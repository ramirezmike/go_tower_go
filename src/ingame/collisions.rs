use bevy::{prelude::*, ecs::system::{Command, SystemState}, };
use crate::AppState;
use bevy_xpbd_3d::{math::*, prelude::*};
use super::{race, bullet, kart};

pub struct CollisionsPlugin;
impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions.run_if(in_state(AppState::InGame)));
    }
}

fn handle_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    mut hit_event_writer: EventWriter<kart::HitEvent>,
    mut bullet_hit_event_writer: EventWriter<bullet::CreateHitEvent>,
    waypoints: Query<(Entity, &race::WayPoint)>,
    waypoint_trackers: Query<(Entity, &race::NextWayPoint)>,
    bullets: Query<(Entity, &bullet::Bullet, &Transform)>,
    karts: Query<(Entity, &kart::Kart)>,
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
                commands.entity(bullet.0).despawn_recursive();
                hit_event_writer.send(kart::HitEvent {
                    entity: kart.0,
                    direction: bullet.1.direction
                });
                bullet_hit_event_writer.send(bullet::CreateHitEvent {
                    position: bullet.2.translation,
                    color: bullet.1.color,
                });
            }

            _ => ()
        }

        // despawn bullets hitting anything
        match (bullets.get(contacts.entity1), bullets.get(contacts.entity2)) {
            (Ok(e), _) | (_, Ok(e)) => {
                bullet_hit_event_writer.send(bullet::CreateHitEvent {
                    position: e.2.translation,
                    color: e.1.color,
                });
                commands.entity(e.0).despawn_recursive();
            },
            _ => ()
        }
    }
}
