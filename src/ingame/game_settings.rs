use bevy::prelude::*;
use crate::{ingame::player, ingame::race::placement_sensor::Place, ingame::race::LapCounter};

#[derive(Resource)]
pub struct GameState {
    pub kart_colors: Vec<Color>,
    pub player_death_cooldown: Timer,
    pub pregame_cooldown: Timer,
    pub ending_state: GameEndingState,
    pub enable_shadows: bool,
    pub enable_background: bool,
    pub enable_extra_physics: bool,
    pub enable_extra_entities: bool,
    pub game_time: f32,
    pub peak_number_of_entities: usize,
    pub player_place: usize,
    pub player_lap: usize,
    pub controller_type: ControllerType,
}

pub enum ControllerType {
    Keyboard,
    Gamepad,
}

#[derive(Default, PartialEq)]
pub enum GameEndingState {
    Winner,
    Died,
    FellBehind,
    #[default]
    Initial,
}

impl GameState {
    pub fn initialize(enable_shadows: bool, enable_background: bool, enable_extra_physics: bool, enable_extra_entities: bool, controller_type: ControllerType) -> Self {
        GameState {
            enable_shadows, enable_background, enable_extra_physics, enable_extra_entities, controller_type,
            ..default()
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            kart_colors: vec!(
                Color::hex("809BCE").unwrap(),
                Color::hex("8E7AAA").unwrap(),
                Color::hex("EAC4D5").unwrap(),
                Color::hex("898D89").unwrap(),
                Color::hex("FFEE93").unwrap(),
                Color::hex("F2CC8F").unwrap(),
                Color::hex("A0E2B1").unwrap(),
                Color::hex("d84546").unwrap(),

            ),
            pregame_cooldown: Timer::from_seconds(6., TimerMode::Once),
            player_death_cooldown: Timer::from_seconds(2., TimerMode::Once),
            ending_state: GameEndingState::Initial,
            enable_shadows: true,
            enable_background: true,
            enable_extra_physics: true,
            enable_extra_entities: false,
            game_time: 0.,
            peak_number_of_entities: 0,
            player_place: 0,
            player_lap: 0,
            controller_type: ControllerType::Keyboard,
        }
    }
}

pub fn update_game_state(
    mut game_state: ResMut<GameState>,
    entities: Query<Entity>,
    player: Query<(&Place, &LapCounter), With<player::Player>>,
    time: Res<Time>
) {
    game_state.peak_number_of_entities = game_state.peak_number_of_entities.max(entities.iter().len());
    game_state.game_time += time.delta_seconds();

    for (place, lap_counter) in &player {
        game_state.player_place = place.0; 
        game_state.player_lap = lap_counter.0;
    }
}
