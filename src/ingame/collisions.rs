use bevy::{prelude::*, ecs::system::{Command, SystemState}, };
use crate::AppState;
use bevy_xpbd_3d::{math::*, prelude::*};
use super::{race, kart};

pub struct CollisionsPlugin;
impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions.run_if(in_state(AppState::InGame)));
    }
}

fn handle_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    waypoints: Query<(Entity, &race::WayPoint)>,
    waypoint_trackers: Query<(Entity, &race::NextWayPoint)>,
    placement_sensors: Query<(Entity, &race::placement_sensor::PlaceSensor)>,
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

        match (placement_sensors.get(contacts.entity1), karts.get(contacts.entity2),
               karts.get(contacts.entity2), placement_sensors.get(contacts.entity1)) {
            (Ok(sensor), Ok(kart), _, _) |
            ( _, _, Ok(kart), Ok(sensor)) => {
                println!("HIT {:?}", kart.0);
            },
            _ => ()
        }

//      match (bullets.get(contacts.entity1), bullets.get(contacts.entity2)) {
//          (Ok(e), _) | (_, Ok(e)) => {
//              commands.entity(e.0).despawn_recursive();
//          },
//          _ => ()
//      }
    }
}
