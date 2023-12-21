use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use crate::{assets, util, AppState, ingame, };
use super::{kart, bullet, config, points, common, player, path};
use bevy_turborand::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use bevy_kira_audio::prelude::*;

#[cfg(feature = "gizmos")]
use bevy::gizmos::gizmos::Gizmos;

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_actions.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component, )]
struct Tower {
    delay_start: Timer,
    action_cooldown: Timer,
    owner: Entity,
    material: Handle<StandardMaterial>,
    target: Vec3,
    color: Color,
}

#[derive(Component)]
struct Cannon {
    parent: Entity,
}

fn tower_actions(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &Transform, &kart::KartColor)>,
    cannons: Query<(Entity, &Cannon)>,
    time: Res<Time>,
    spatial_query: SpatialQuery,

    #[cfg(feature = "gizmos")]
    mut gizmos: Gizmos,
) {
    for (tower_entity, mut tower, tower_transform, kart_color) in &mut towers {
        if !tower.delay_start.tick(time.delta()).finished() {
            continue;
        }

        if tower.action_cooldown.tick(time.delta()).just_finished() {
            let spawn_point = tower_transform.translation + Vec3::new(0., config::TOWER_HEIGHT, 0.);
            for (cannon_entity, cannon) in &cannons {
                if cannon.parent == tower_entity {
                    commands.entity(cannon_entity)
                        .insert(common::scaler::Scaler::new(Vec3::splat(1.2), 0.1, 0.4, false));
                }
            }

            commands.add(bullet::BulletSpawner {
                owner: tower.owner,
                spawn_point,
                material: tower.material.clone_weak(),
                direction: tower.target - spawn_point,
                color: tower.color,
                kart_color: *kart_color,
                speed: 2.0,
                cleanup_marker: ingame::CleanupMarker,
            });
        } 

        #[cfg(feature = "gizmos")]
        {
            let bullet_spawn_point = tower_transform.translation + Vec3::new(0., config::TOWER_HEIGHT, 0.);
            gizmos.sphere(bullet_spawn_point, Quat::IDENTITY, 1., Color::RED);
            let rays_to_cast = vec!(
                tower_transform.translation + Vec3::new(-config::TRACK_WIDTH, 5.0, 0.0),
                tower_transform.translation + Vec3::new(config::TRACK_WIDTH, 5.0, 0.0),
                tower_transform.translation + Vec3::new(0.0, 5.0, -config::TRACK_WIDTH),
                tower_transform.translation + Vec3::new(0.0, 5.0, config::TRACK_WIDTH),
            );

            for ray in rays_to_cast {
                if let Some(first_hit) = spatial_query.cast_ray(
                    ray,
                    -Vec3::Y,
                    100.0,
                    true,
                    SpatialQueryFilter::default(),
                ) {
                    gizmos.line(ray, ray + Vec3::new(0., first_hit.time_of_impact, 0.), Color::RED);
                } else {
                    gizmos.line(ray, ray + Vec3::new(0., -20., 0.), Color::GREEN);
                }
            }
        }
    }
}

pub struct CannonSpawner {
    parent: Entity,
    spawn_point: Vec3,
    target: Vec3,
    outline_color: Color,
    outline_width: f32
}

impl Command for CannonSpawner {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            Res<assets::GameAssets>,
            Res<Assets<Gltf>>,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, assets_gltf) = system_state.get_mut(world);

        let material = assets_handler.materials.add(Color::rgb(0.5, 0.5, 0.5).into()).clone();
        let mesh = game_assets.cannon.mesh.clone_weak(); 
        world.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(self.spawn_point + Vec3::new(0., config::TOWER_HEIGHT - 1., 0.))
                            .looking_at(self.target - Vec3::new(0., config::TOWER_HEIGHT - 1., 0.), Vec3::Y)
                            .with_scale(Vec3::splat(0.1)),
                ..default()
            },
            common::scaler::Scaler::new(Vec3::splat(1.0), 0.5, 0.0, true),
            ingame::CleanupMarker,
            Cannon {
                parent: self.parent,
            },
            OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    width: self.outline_width,
                    colour: self.outline_color,
                },
                mode: OutlineMode::RealVertex,
                ..default()
            })
        );
    }
}

pub struct TowerSpawner {
    pub entity: Entity,
    pub material: Handle<StandardMaterial>,
}
impl Command for TowerSpawner {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            Res<assets::GameAssets>,
            Res<Assets<Gltf>>,
            SpatialQuery,
            ResMut<GlobalRng>,
            Res<path::PathManager>,
            Res<Audio>,
            Query<(&Transform, &kart::Kart, &kart::KartColor, &mut points::Points, Has<player::Player>)>,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, assets_gltf, spatial_query, mut global_rng, path_manager, audio, mut points) = system_state.get_mut(world);

        if let Ok((transform, kart, kart_color, mut point, is_player)) = points.get_mut(self.entity) {
            let spawn_point = transform.translation;
            let cost = if cfg!(feature = "endless") { 0 } else { 4 }; 
            if point.0 >= cost {
                let color = kart.0;
                let gltf = assets_gltf.get(&game_assets.tower_01);
                if let Some(gltf) = gltf {
                    let scene = gltf.scenes[0].clone_weak();
                    let starting_height = 5.0;

                    let check_point = 
                        if is_player { 
                            path_manager.get_closest_index(spawn_point)
                                        .and_then(|i| path_manager.get_next(i + 1))
                                        .map(|i| path_manager.get(i))
                                        .map(|v| spawn_point.lerp(v, 0.45))
                                        .unwrap_or(spawn_point)
                        } else {
                            spawn_point
                        };

                    let rays_to_cast = vec!(
                        check_point + Vec3::new(-config::TRACK_WIDTH, starting_height, 0.0),
                        check_point + Vec3::new(config::TRACK_WIDTH, starting_height, 0.0),
                        check_point + Vec3::new(0.0, starting_height, -config::TRACK_WIDTH),
                        check_point + Vec3::new(0.0, starting_height, config::TRACK_WIDTH),

                        // just in case?
                        check_point + Vec3::new(config::TRACK_WIDTH, starting_height, config::TRACK_WIDTH),
                        check_point + Vec3::new(config::TRACK_WIDTH, starting_height, -config::TRACK_WIDTH),
                        check_point + Vec3::new(-config::TRACK_WIDTH, starting_height, -config::TRACK_WIDTH),
                        check_point + Vec3::new(-config::TRACK_WIDTH, starting_height, config::TRACK_WIDTH),

                        // uhh ok one more just in case?
                        check_point + Vec3::new(-config::TRACK_WIDTH - 1., starting_height, 0.0),
                        check_point + Vec3::new(config::TRACK_WIDTH + 1., starting_height, 0.0),
                        check_point + Vec3::new(0.0, starting_height, -config::TRACK_WIDTH - 1.),
                        check_point + Vec3::new(0.0, starting_height, config::TRACK_WIDTH + 1.),

                        check_point + Vec3::new(config::TRACK_WIDTH + 1., starting_height, config::TRACK_WIDTH + 1.),
                        check_point + Vec3::new(config::TRACK_WIDTH + 1., starting_height, -config::TRACK_WIDTH - 1.),
                        check_point + Vec3::new(-config::TRACK_WIDTH - 1., starting_height, -config::TRACK_WIDTH - 1.),
                        check_point + Vec3::new(-config::TRACK_WIDTH - 1., starting_height, config::TRACK_WIDTH + 1.),
                    );

                    for ray in rays_to_cast {
                        let hit = spatial_query.cast_ray(
                            ray,
                            -Vec3::Y,
                            10.0,
                            true,
                            SpatialQueryFilter::default(),
                        );

                        if hit.is_none() {
                            let original_offset = ray - check_point;
                            let normalized_offset= (original_offset - Vec3::new(0., starting_height, 0.)).normalize();
                            let buffered_position = config::TRACK_WIDTH + config::TOWER_POSITION_BUFFER;
                            let offset_with_buffer = normalized_offset * Vec3::new(buffered_position, 0., buffered_position);
                            let target = spawn_point;
                            let spawn_point = check_point + offset_with_buffer;

                            point.0 -= cost;
                            let random = global_rng.f32();
                            let tower_color = kart.0;
                            let kart_color = kart_color.clone();

                            let sfx = audio.play(game_assets.sfx_tower.clone()).with_volume(0.).handle();

                            let tower_id = world.spawn((
                                Tower {
                                    target,
                                    owner: self.entity,
                                    material: self.material,
                                    color,
                                    delay_start: Timer::from_seconds(random, TimerMode::Once),
                                    action_cooldown:Timer::from_seconds(0.5, TimerMode::Repeating), 
                                },
                                kart_color,
                                AudioEmitter {
                                    instances: vec![sfx],
                                },
                                ingame::CleanupMarker,
                                common::scaler::Scaler::new(Vec3::splat(1.0), 0.5, 0.0, true),
                                util::scene_hook::HookedSceneBundle {
                                    scene: SceneBundle {
                                        scene,
                                        transform: Transform::from_translation(spawn_point).with_scale(Vec3::splat(0.1)),
                                        ..default()
                                    },
                                    hook: util::scene_hook::SceneHook::new(move |cmds, hook_data| {
                                        if let (Some(mesh), Some(name)) = (hook_data.mesh, hook_data.name) {
                                            cmds.insert(
                                            OutlineBundle {
                                                outline: OutlineVolume {
                                                    visible: true,
                                                    width: if is_player { 8.0 } else { 1.0 },
                                                    colour: if is_player { tower_color } else { Color::BLACK },
                                                },
                                                mode: OutlineMode::RealVertex,
                                                ..default()
                                            });
                                        }
                                    })
                                }, 
                            )).id();

                            let cannon_spawner = CannonSpawner {
                                parent: tower_id,
                                spawn_point,
                                outline_color: if is_player { tower_color } else { Color::BLACK },
                                outline_width: if is_player { 8.0 } else { 1.0 },
                                target
                            };
                            cannon_spawner.apply(world);
                            break;
                        } 
                    }
                }
            } 
        }
    }
}
