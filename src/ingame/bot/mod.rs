use bevy::prelude::*;
use crate::{AppState, };
use super::{controller, path};
use bevy_xpbd_3d::prelude::*;
use bevy_turborand::prelude::*;

#[cfg(feature = "gizmos")]
use bevy::gizmos::gizmos::Gizmos;

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (find_target, move_bots).chain().run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component, Default)]
pub struct Bot {
    target: Option<usize>,
    random: f32,
}

impl Bot {
    pub fn new(random: f32) -> Self {
        Bot {
            random,
            ..default()
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
                //println!("Dot is {:?}, turning left", dot);
                movement_event_writer.send(controller::MovementEvent {
                    entity,
                    action: controller::MovementAction::Turn(0.1 * dot.abs() + bot.random),
                });
            } else if dot > 0. {
                //println!("Dot is {:?}, turning right", dot);
                movement_event_writer.send(controller::MovementEvent {
                    entity,
                    action: controller::MovementAction::Turn(-0.1 * dot.abs() + bot.random),
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
