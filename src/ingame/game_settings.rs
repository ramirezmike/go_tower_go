use bevy::prelude::*;

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
}

#[derive(Default)]
pub enum GameEndingState {
    Winner,
    Died,
    FellBehind,
    #[default]
    Initial,
}

impl GameState {
    pub fn initialize(enable_shadows: bool, enable_background: bool, enable_extra_physics: bool, enable_extra_entities: bool) -> Self {
        GameState {
            enable_shadows, enable_background, enable_extra_physics, enable_extra_entities,
            ..default()
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            kart_colors: vec!(
                Color::hex("809BCE").unwrap(),
                Color::hex("DEDEDE").unwrap(),
                Color::hex("EAC4D5").unwrap(),
                Color::hex("898D89").unwrap(),
                Color::hex("FFEE93").unwrap(),
                Color::hex("F2CC8F").unwrap(),
                Color::hex("A0E2B1").unwrap(),
                Color::hex("8E7AAA").unwrap(),

            ),
            pregame_cooldown: Timer::from_seconds(5., TimerMode::Once),
            player_death_cooldown: Timer::from_seconds(2., TimerMode::Once),
            ending_state: GameEndingState::Initial,
            enable_shadows: true,
            enable_background: true,
            enable_extra_physics: true,
            enable_extra_entities: true,
        }
    }
}
