use bevy::{prelude::*, ecs::{system::{Command, SystemState}, }, };
use crate::{AppState, IngameState, ingame::path, ingame::race, ingame::player, ingame::common, ingame::game_settings, util::audio};
use std::collections::HashMap;
use bevy_xpbd_3d::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::util::num_ext::*;

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

        let mut index =
            if let Some(index) = substring.chars().rev().take_while(|&c| c.is_digit(10)).collect::<String>().chars().rev().collect::<String>().parse::<usize>().ok() {

                index
            } else {
                // there should only be one of these
                0
            };
//      for _ in 0..40 {
//          index = index.circular_decrement(0, 377);
//      }

        PlaceSensor {
            index
        }
    }
}

fn detect_racer_places(
    mut commands: Commands,
    place_sensor: Query<&PlaceSensor>,
    mut racers: Query<(Entity, &Transform, &race::LapCounter, &mut race::PlaceCounter, Has<player::Player>)>,
    spatial_query: SpatialQuery,
    mut health_hit_event_writer: EventWriter<common::health::HealthHitEvent>,
    mut game_state: ResMut<game_settings::GameState>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    mut game_audio: audio::GameAudio,
    mut audio: Res<Audio>,

    #[cfg(feature = "gizmos")]
    mut gizmos: Gizmos,
) {
    let mut sensed_racers = vec!();
    let mut furthest_place = HashMap::<usize, usize>::default();
    let mut furthest_lap= 0;
    for (racer_entity, transform, lap_counter, mut place_counter, is_player) in &mut racers {
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
                let mut current_place = place_sensor.index;
                if let Some(entry) = furthest_place.get_mut(&lap_counter.0) {
                    *entry = *entry.max(&mut current_place);
                } else {
                    furthest_place.insert(place_sensor.index, lap_counter.0);
                }

                if place_counter.0 < current_place && current_place - place_counter.0 < 20 {
                    place_counter.0 = current_place;
                }

                furthest_lap = furthest_lap.max(lap_counter.0);
            }
        } 

        sensed_racers.push((racer_entity, lap_counter.0, place_counter.0, is_player));
    }

    sensed_racers.sort_by_key(|(_, lap, place, _)| (lap * 1000) + place);
    for (i, (e, lap, _, is_player)) in sensed_racers.iter().rev().enumerate() {
        commands.entity(*e).insert(Place(i + 1));

        if *lap < furthest_lap.saturating_sub(1) { // kart fell behind
            #[cfg(not(feature = "endless"))]
            {
                health_hit_event_writer.send(common::health::HealthHitEvent {
                    entity: *e,
                    hit_points: 10
                });

                if *is_player {
                    audio.stop();
                    game_audio.stop_bgm();
                    game_state.ending_state = game_settings::GameEndingState::FellBehind;
                    next_ingame_state.set(IngameState::EndGame);
                }
            }
        }
    }
}
