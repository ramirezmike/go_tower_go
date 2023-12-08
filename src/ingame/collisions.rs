use bevy::{prelude::*, ecs::system::{Command, SystemState}, };
use crate::{AppState, util};
use bevy_xpbd_3d::{math::*, prelude::*};
use super::{race, bullet, kart, player, common::{self, health::Invulnerability}, config, game_settings};

pub struct CollisionsPlugin;
impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions.run_if(in_state(AppState::InGame)));
    }
}

#[derive(PhysicsLayer)]
pub enum Layer {
    Kart,
    Ground,
    Bullet,
}


fn handle_collisions(
    mut commands: Commands,
    mut game_state: ResMut<game_settings::GameState>,
    mut collision_event_reader: EventReader<Collision>,
    mut hit_event_writer: EventWriter<kart::HitEvent>,
    mut bullet_hit_event_writer: EventWriter<bullet::CreateHitEvent>,
    mut health_hit_event_writer: EventWriter<common::health::HealthHitEvent>,
    waypoints: Query<(Entity, &race::WayPoint)>,
    waypoint_trackers: Query<(Entity, &race::NextWayPoint)>,
    bullets: Query<(Entity, &bullet::Bullet, &Transform)>,
    karts: Query<(Entity, &kart::Kart, Has<player::Player>), Without<Invulnerability>>,
    tracks: Query<(Entity, With<super::Track>)>
) {
    for Collision(contacts) in collision_event_reader.read() {
        match (waypoint_trackers.get(contacts.entity1), waypoints.get(contacts.entity2),
               waypoint_trackers.get(contacts.entity2), waypoints.get(contacts.entity1)) {
            (Ok(next_waypoint), Ok(waypoint), _, _) |
            ( _, _, Ok(next_waypoint), Ok(waypoint)) => {
                // check if hit waypoint is targeted by entity
                if waypoint.1.0 == next_waypoint.1.0 {
                    commands.add(race::WayPointHitHandler {
                        entity: next_waypoint.0
                    });
                }
            },
            _ => ()
        }

        match (bullets.get(contacts.entity1), karts.get(contacts.entity2),
               bullets.get(contacts.entity2), karts.get(contacts.entity1)) {
            (Ok(bullet), Ok(kart), _, _) | 
            (_, _, Ok(bullet), Ok(kart)) => {
                if bullet.1.owner != kart.0 {
                    commands.entity(bullet.0).despawn_recursive();
                    hit_event_writer.send(kart::HitEvent {
                        entity: kart.0,
                        direction: bullet.1.direction
                    });
                    health_hit_event_writer.send(common::health::HealthHitEvent {
                        entity: kart.0,
                        hit_points: 1
                    });
                    bullet_hit_event_writer.send(bullet::CreateHitEvent {
                        position: bullet.2.translation,
                        count: config::BULLET_HIT_COUNT,
                        with_physics: game_state.enable_extra_physics,
                        color: bullet.1.color,
                    });

                    if kart.2 { // is player
                        commands.add(util::screen_shake::CameraShake::default());
                    }
                }
            }

            _ => ()
        }

        // despawn bullets hitting track
        match (bullets.get(contacts.entity1), tracks.get(contacts.entity2),
               bullets.get(contacts.entity2), tracks.get(contacts.entity1)) {
            (Ok(bullet), Ok(track), _, _) | 
            (_, _, Ok(bullet), Ok(track)) => {
                bullet_hit_event_writer.send(bullet::CreateHitEvent {
                    position: bullet.2.translation,
                    count: config::BULLET_HIT_COUNT,
                    with_physics: game_state.enable_extra_physics,
                    color: bullet.1.color,
                });
                commands.entity(bullet.0).despawn_recursive();
            },
            _ => ()
        }
    }
}
