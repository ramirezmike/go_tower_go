use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use crate::{assets, util, AppState, };
use super::{kart, bullet, config};
use bevy_xpbd_3d::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};

#[cfg(feature = "gizmos")]
use bevy::gizmos::gizmos::Gizmos;

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_actions.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component, Default)]
struct Tower {
    action_cooldown: Timer,
    target: Vec3,
}

fn tower_actions(
    mut commands: Commands,
    mut towers: Query<(&mut Tower, &Transform), Without<kart::Kart>>,
    karts: Query<(&kart::Kart, &Transform), Without<Tower>>,
    time: Res<Time>,
    spatial_query: SpatialQuery,

    #[cfg(feature = "gizmos")]
    mut gizmos: Gizmos,
) {
    for (mut tower, tower_transform) in &mut towers {
        if tower.action_cooldown.tick(time.delta()).just_finished() {
            let spawn_point = tower_transform.translation + Vec3::new(0., config::TOWER_HEIGHT, 0.);
            commands.add(bullet::BulletSpawner {
                spawn_point,
                direction: tower.target - spawn_point,
                speed: 2.0,
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

pub struct TowerSpawner {
    pub spawn_point: Vec3,
}
impl Command for TowerSpawner {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            Res<assets::GameAssets>,
            Res<Assets<Gltf>>,
            SpatialQuery,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, assets_gltf, spatial_query) = system_state.get_mut(world);
        let gltf = assets_gltf.get(&game_assets.tower_01);
        if let Some(gltf) = gltf {
            let scene = gltf.scenes[0].clone();
            let starting_height = 5.0;

            let rays_to_cast = vec!(
                self.spawn_point + Vec3::new(-config::TRACK_WIDTH, starting_height, 0.0),
                self.spawn_point + Vec3::new(config::TRACK_WIDTH, starting_height, 0.0),
                self.spawn_point + Vec3::new(0.0, starting_height, -config::TRACK_WIDTH),
                self.spawn_point + Vec3::new(0.0, starting_height, config::TRACK_WIDTH),
            );

            for ray in rays_to_cast {
                let hit = spatial_query.cast_ray(
                    ray,
                    -Vec3::Y,
                    100.0,
                    true,
                    SpatialQueryFilter::default(),
                );

                if hit.is_none() {
                    let original_offset = ray - self.spawn_point;
                    let normalized_offset= (original_offset - Vec3::new(0., starting_height, 0.)).normalize();
                    let buffered_position = config::TRACK_WIDTH + config::TOWER_POSITION_BUFFER;
                    let offset_with_buffer = normalized_offset * Vec3::new(buffered_position, 0., buffered_position);
                    let spawn_point = self.spawn_point + offset_with_buffer;

                    world.spawn((
                        Tower {
                            target: self.spawn_point,
                            action_cooldown:Timer::from_seconds(1.0, TimerMode::Repeating), 
                        },
                        util::scene_hook::HookedSceneBundle {
                            scene: SceneBundle {
                                scene,
                                transform: Transform::from_translation(spawn_point),
                                ..default()
                            },
                            hook: util::scene_hook::SceneHook::new(move |cmds, hook_data| {
                                if let (Some(mesh), Some(name)) = (hook_data.mesh, hook_data.name) {
                                    if name.contains("collide") {
                                    }

                                    cmds.insert(
                                    OutlineBundle {
                                        outline: OutlineVolume {
                                            visible: true,
                                            width: 1.0,
                                            colour: Color::BLACK,
                                        },
                                        mode: OutlineMode::RealVertex,
                                        ..default()
                                    });
                                }
                            })
                        }, 
                    ));
                    break;
                }
            }
        }
    }
}
