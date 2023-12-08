use bevy::{prelude::*, ecs::{system::{Command, SystemState}, }, };
use crate::{AppState, IngameState, ingame::path, ingame::race, ingame::player, ingame::common, ingame::game_settings};
use bevy_xpbd_3d::prelude::*;

#[cfg(feature = "gizmos")]
use bevy::gizmos::gizmos::Gizmos;

pub struct PlacementSensorPlugin;
impl Plugin for PlacementSensorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                FixedUpdate,
                (detect_racer_places,).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Place(pub usize);

#[derive(Component, Default)]
pub struct PlaceSensor {
    index: usize
}

impl PlaceSensor {
    pub fn new(name: &str) -> Self {
        let last_dot_index = name.rfind('.').expect(&format!("Name {} for Place sensor not formatted correctly", name));
        let substring = &name[last_dot_index..];

        let index =
            if let Some(index) = substring.chars().rev().take_while(|&c| c.is_digit(10)).collect::<String>().chars().rev().collect::<String>().parse::<usize>().ok() {

                index
            } else {
                // there should only be one of these
                0
            };

        PlaceSensor {
            index
        }
    }
}

fn detect_racer_places(
    mut commands: Commands,
    place_sensor: Query<&PlaceSensor>,
    racers: Query<(Entity, &Transform, &race::LapCounter, Has<player::Player>)>,
    spatial_query: SpatialQuery,
    mut health_hit_event_writer: EventWriter<common::health::HealthHitEvent>,
    mut game_state: ResMut<game_settings::GameState>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,

    #[cfg(feature = "gizmos")]
    mut gizmos: Gizmos,
) {
    let mut sensed_racers = vec!();
    let mut furthest_place = 0;
    let mut furthest_lap= 0;
    for (racer_entity, transform, lap_counter, is_player) in &racers {
        let hit = spatial_query.cast_ray(
            transform.translation + Vec3::new(0., -10., 0.),
            -Vec3::Y,
            100.0,
            true,
            SpatialQueryFilter::default(),
        );

        #[cfg(feature = "gizmos")]
        {
            gizmos.line(transform.translation + Vec3::new(0., -10., 0.), transform.translation + Vec3::new(0., -10., 0.) + Vec3::new(0., -100., 0.), Color::RED);
        }

        if let Some(hit) = hit {
            if let Ok(place_sensor) = place_sensor.get(hit.entity) {
                furthest_place =  furthest_place.max(place_sensor.index);
                furthest_lap = furthest_lap.max(lap_counter.0);
                sensed_racers.push((racer_entity, lap_counter.0, place_sensor.index, is_player));
            }
        }
    }

    sensed_racers.sort_by_key(|(_, lap, place, _)| (lap * furthest_place) + place);
    for (i, (e, lap, _, is_player)) in sensed_racers.iter().rev().enumerate() {
        commands.entity(*e).insert(Place(i + 1));

        if *lap < furthest_lap.saturating_sub(1) { // kart fell behind
            health_hit_event_writer.send(common::health::HealthHitEvent {
                entity: *e,
                hit_points: 10
            });

            if *is_player {
                game_state.ending_state = game_settings::GameEndingState::FellBehind;
                next_ingame_state.set(IngameState::EndGame);
            }
        }
    }
}
