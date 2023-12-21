use bevy::prelude::*;
use crate::{AppState, IngameState};
use bevy_turborand::prelude::*;
use super::{controller, path, race, tower, kart, assets};
use bevy_xpbd_3d::prelude::*;

#[cfg(feature = "gizmos")]
use bevy::gizmos::gizmos::Gizmos;

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (find_target, move_bots).chain()
                        .run_if(in_state(AppState::InGame).and_then(in_state(IngameState::InGame)))
                        )
            .add_systems(
                FixedUpdate,
                ((place_towers, ).chain()).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component, Default)]
pub struct Bot {
    target: Option<usize>,
    spawn_delay: Timer,
    random: f32,
}

impl Bot {
    pub fn new(random: f32) -> Self {
        Bot {
            random,
            spawn_delay: Timer::from_seconds(1., TimerMode::Once),
            ..default()
        }
    }
}

#[derive(Bundle)]
pub struct BotBundle {
    bot: Bot,
    tower_placer: TowerPlacer, 
}

impl BotBundle {
    pub fn new(normalized_rand: f32, positive_rand: f32) -> Self {
        BotBundle {
            bot: Bot::new(normalized_rand),
            tower_placer: TowerPlacer::new(positive_rand)
        }
    }
}

#[derive(Component, Default)]
pub struct TowerPlacer {
    min_percentage_into_track: f32,
}

impl TowerPlacer {
    pub fn new(random: f32) -> Self {
        TowerPlacer {
            min_percentage_into_track: 0.1 + (random % 0.8),
            ..default()
        }
    }
}

fn place_towers(
    mut commands: Commands,
    mut bots: Query<(Entity, &mut Bot, &mut TowerPlacer, &kart::Kart, &kart::KartColor)>,
    game_assets: Res<assets::GameAssets>,
    waypoints: Query<&race::WayPoint>,
    mut global_rng: ResMut<GlobalRng>,
    path_manager: Res<path::PathManager>,
    time: Res<Time>,
) {
    for (entity, mut b, mut tower_placer, kart, kart_color) in &mut bots {
        if !b.spawn_delay.tick(time.delta()).finished() {
            continue;
        }

        if let Some(bot_index) = b.target {
            let waypoints =
            waypoints.iter()
                     .fold((None, None), |mut acc, e| {
                         if e.0 == race::WayPoints::Start && e.1.is_some() {
                             acc.0 = e.1;
                         }
                         if e.0 == race::WayPoints::Finish && e.1.is_some() {
                             acc.1 = e.1;
                         }

                         acc
                     });
            if let (Some(start_index), Some(mut finish_index)) = waypoints {

                if finish_index == 0 {
                    finish_index = path_manager.get_previous(finish_index).unwrap_or(20); // just picking something
                }

                if bot_index < start_index || bot_index > finish_index {
                    continue;
                }

                if (bot_index - start_index) as f32 / finish_index as f32 > tower_placer.min_percentage_into_track {
                    commands.add(tower::TowerSpawner {
                        entity,
                        material: game_assets.kart_colors[&kart_color.0].clone_weak(),
                    });

                    b.spawn_delay = Timer::from_seconds(1., TimerMode::Once);
                    tower_placer.min_percentage_into_track = 0.1 + (global_rng.f32() % 0.8);
                } 
            }
        }
    }
}

fn find_target(
    mut bots: Query<(&mut Bot, &Transform)>,
    path_manager: Res<path::PathManager>,
) {
    for (mut bot, transform) in &mut bots {
        match bot.target {
            Some(target) => {
                let target_point = path_manager.get(target);
                let distance = transform.translation.distance(target_point);
                if distance < 20.0 {
                    bot.target = path_manager.get_next(target);
                }
            },
            None => {
                bot.target = path_manager.get_closest_index(transform.translation);
            }
        }
    }
}

fn move_bots(
    bots: Query<(Entity, &Bot, &Transform, &LinearVelocity)>,
    path_manager: Res<path::PathManager>,
    mut movement_event_writer: EventWriter<controller::MovementEvent>,

    #[cfg(feature = "gizmos")]
    mut gizmos: Gizmos,
) {
    for (entity, bot, transform, linear_velocity) in &bots {
        if let Some(target) = bot.target {
            let target = path_manager.get(target);

            #[cfg(feature = "gizmos")]
            {
                gizmos.sphere(target.clone() + Vec3::new(0., 0.5, 0.), Quat::IDENTITY, 1., Color::YELLOW);
                let normalized_velocity = linear_velocity.0.normalize() * 5.0;
                gizmos.line(transform.translation, transform.translation + normalized_velocity, Color::GREEN);
                gizmos.line(transform.translation, transform.translation + (transform.forward() * 5.0), Color::RED);
            }

            let difference = target - (transform.translation + linear_velocity.0);
            let dot = transform.right().dot(difference);

            if dot < -0. {
                movement_event_writer.send(controller::MovementEvent {
                    entity,
                    action: controller::MovementAction::Turn(0.1 * dot.abs() + (bot.random * 0.6)),
                });
            } else if dot > 0. {
                movement_event_writer.send(controller::MovementEvent {
                    entity,
                    action: controller::MovementAction::Turn(-0.1 * dot.abs() + (bot.random * 0.6)),
                });
            }

//            if dot.abs() < 10.0 {
                movement_event_writer.send(controller::MovementEvent {
                    entity,
                    action: controller::MovementAction::Gas,
                });
//            } 

        } else {
            movement_event_writer.send(controller::MovementEvent {
                entity,
                action: controller::MovementAction::Brake,
            });
        }
    }
}
