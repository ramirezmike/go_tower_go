use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use bevy::render::primitives::Aabb;
use bevy_turborand::prelude::*;
use std::f32::consts::TAU;
use bevy_xpbd_3d::{math::*, prelude::*};
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use crate::{assets, util, AppState, IngameState};
use super::{bot, controller, player, config, race, points, game_settings, particle, common, CleanupMarker, bullet};
use bevy_xpbd_3d::PhysicsSet;
use bevy::transform::TransformSystem;

#[cfg(feature = "gizmos")]
use bevy::gizmos::gizmos::Gizmos;

pub struct KartPlugin;
impl Plugin for KartPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent>()
            .add_systems(Update, (spawn_smoke, handle_hits, upright_karts, ).run_if(in_state(AppState::InGame)))

            .add_systems(
                PostUpdate,
                handle_deaths
                    .run_if(in_state(AppState::InGame))
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component)]
pub struct Kart(pub Color);

#[derive(Event)]
pub struct HitEvent {
    pub entity: Entity,
    pub direction: Vec3
}

#[derive(Component)]
pub struct Smoker {
    cooldown: Timer,
}

impl Default for Smoker {
    fn default() -> Self {
        Smoker {
            cooldown: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

fn handle_deaths(
    mut commands: Commands,
    karts: Query<(Entity, &Transform, &common::health::Health, &Kart,Has<player::Player>), >,
    mut bullet_hit_event_writer: EventWriter<bullet::CreateHitEvent>,
    time: Res<Time>,
    mut game_state: ResMut<game_settings::GameState>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    mut current_state: ResMut<State<IngameState>>,
) {
    let mut player_is_dead = false;
    let mut player_exists= false;
    for (entity, transform, health, kart, is_player) in &karts {
        if health.is_dead() {
            bullet_hit_event_writer.send(bullet::CreateHitEvent {
                position: transform.translation,
                count: config::KART_DIE_HIT_COUNT,
                with_physics: true,
                color: kart.0,
            });
            commands.entity(entity).despawn_recursive();
        }

        if is_player {
            player_is_dead = health.is_dead(); 
            player_exists = true;
        }
    }
    
    if *current_state.get() == IngameState::InGame{
        let player_won = player_exists && karts.iter().len() <= 1;
        if ((!player_exists || player_is_dead) || player_won) && game_state.player_death_cooldown.tick(time.delta()).finished() {
            game_state.is_winner = player_won;
            next_ingame_state.set(IngameState::EndGame);
        }
    }
}

fn upright_karts(
    karts: Query<(Entity, &Transform, &Children) , With<Kart>>,
    mut other_transforms: Query<&mut Transform, Without<Kart>>,
    spatial_query: SpatialQuery,

    #[cfg(feature = "gizmos")]
    mut gizmos: Gizmos,
) {
    for (entity, kart_transform, children) in &karts {
        for child in children {
            if let Ok(mut child_transform) = other_transforms.get_mut(*child) {
                #[cfg(feature = "gizmos")]
                {
                    gizmos.line(kart_transform.translation, kart_transform.translation + Vec3::new(0., -10., 0.), Color::PURPLE);
                }

                let hits = spatial_query.ray_hits(
                    kart_transform.translation ,
                    -Vec3::Y,
                    10.0,
                    5,
                    false, SpatialQueryFilter::default(),
                );

                for hit in hits {
                    if entity == hit.entity {
                        continue;
                    }
                    
                    let inverse_parent_rotation = kart_transform.rotation.inverse();
                    let inverted_normal = inverse_parent_rotation.mul_vec3(hit.normal);
                    child_transform.rotation = Quat::from_rotation_arc(Vec3::Y, inverted_normal);
                }
            }
        }
    }
}

fn spawn_smoke(
    mut smoke_event_writer: EventWriter<particle::CreateParticleEvent>,
    mut smokers: Query<(&mut Smoker, &Transform, &LinearVelocity, Has<controller::Braking>), With<controller::Grounded>>, 
    time: Res<Time>,
) {
    for (mut smoker, transform, linear_velocity, is_braking) in &mut smokers {
        if smoker.cooldown.tick(time.delta()).just_finished() {
            if linear_velocity.0.length() > 1. && (is_braking || linear_velocity.0.angle_between(transform.forward()) > 0.4) {
                smoke_event_writer.send(particle::CreateParticleEvent {
                    position: *transform
                });
            }
        }
    }
}

fn handle_hits(
    mut hit_event_reader: EventReader<HitEvent>,
    mut karts: Query<&mut LinearVelocity, With<Kart>>,
) {
    for event in hit_event_reader.read() {
        if let Ok(mut velocity) = karts.get_mut(event.entity) {
            velocity.0 = event.direction.normalize() * 10.0; 
        }
    }
}

pub struct KartSpawner<C: Component + Clone> {
    pub global_transform: GlobalTransform,
    pub aabb: Aabb,
    pub cleanup_marker: C
}
impl<C: Component + Clone> Command for KartSpawner<C> {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            Res<assets::GameAssets>,
            Res<Assets<Gltf>>,
            ResMut<GlobalRng>,
            ResMut<game_settings::GameState>,
            Query<Entity, With<player::Player>>,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, assets_gltf, mut global_rng, mut game_state, players) = system_state.get_mut(world);
        let matrix = self.global_transform.compute_matrix();
        let spawn_point = matrix.transform_point3(self.aabb.center.into());
        let rand = global_rng.f32_normalized();
        let positive_rand = global_rng.f32();

        let count_of_spawned_players = players.iter().count();
        let color = game_state.kart_colors.pop().expect("Ran out of colors for the karts");
        let color_material = assets_handler.materials.add(color.into());

        #[cfg(feature = "no_bots")]
        {
            if count_of_spawned_players >= config::NUMBER_OF_PLAYERS {
                return;
            }
        }

        let gltf = assets_gltf.get(&game_assets.car);
        if let Some(gltf) = gltf {
            let scene = gltf.scenes[0].clone();
            let mut entity = world.spawn(Kart(color));
            let kart_id = entity.id();
            entity.insert((
                Kart(color),
                util::scene_hook::HookedSceneBundle {
                    scene: SceneBundle {
                        scene,
                        transform: Transform::from_translation(spawn_point + Vec3::new(0., 0.5, 0.)).with_rotation(Quat::from_axis_angle(Vec3::Y, TAU * 0.75)),
                        ..default()
                    },
                    hook: util::scene_hook::SceneHook::new(move |cmds, hook_data| {
                        if let Some(_) = hook_data.mesh {
                            cmds.insert((
                            OutlineBundle {
                                outline: OutlineVolume {
                                    visible: true,
                                    width: 1.0,
                                    colour: Color::BLACK,
                                },
                                mode: OutlineMode::RealVertex,
                                ..default()
                            }));
                        }

                        if let Some(name) = hook_data.name {
                            if name.contains("color") {
                                cmds.insert(color_material.clone());
                            }
                        }
                    })
                },
                race::NextWayPoint(race::WayPoints::Quarter),
                race::LapCounter(1),
                points::Points(8),
                Smoker::default(), 
                self.cleanup_marker,
                //controller::CommonControllerBundle::new(Collider::capsule(0.3, 0.5), Vector::NEG_Y * 9.81 * 1.5)
                controller::CommonControllerBundle::new(Collider::cuboid(1.5, 1.0, 1.5), Vector::NEG_Y * 9.81 * 1.5),
            ));


            if count_of_spawned_players >= config::NUMBER_OF_PLAYERS {
                entity.insert(bot::BotBundle::new(rand, positive_rand));
            } else {
                entity.insert((player::Player, controller::CharacterControllerKeyboard,));
            }

            common::health::HealthBarSpawner::<CleanupMarker> {
                health_points: config::KART_HEALTH,
                parent: kart_id, 
                cleanup_marker: CleanupMarker,
                offset: Vec3::new(0., 2.0, 0.),
            }.apply(world);
        }
    }
}
