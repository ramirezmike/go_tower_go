use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub kart_colors: Vec<Color>,
    pub player_death_cooldown: Timer,
    pub pregame_cooldown: Timer,
    pub is_winner: bool
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
            is_winner: false,
        }
    }
}
