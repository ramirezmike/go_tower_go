use crate::{cleanup, AppState};
use bevy::prelude::*;

pub mod loader;
mod setup;
mod update;

use self::{
    setup::setup,
    update::handle_input
};

pub struct InstructionsPlugin;
impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Instructions), setup)
            .init_resource::<InstructionsState>()
            .add_systems(
                Update, handle_input.run_if(in_state(AppState::Instructions)),
            )
            .add_systems(OnExit(AppState::Instructions), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
struct CleanupMarker;
#[derive(Resource)]
pub struct InstructionsState {
    pub cooldown: Timer
}

impl Default for InstructionsState {
    fn default() -> Self {
        InstructionsState {
            cooldown: Timer::from_seconds(0.2, TimerMode::Once)
        }
    }
}

