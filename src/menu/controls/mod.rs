use crate::{cleanup, AppState};
use bevy::prelude::*;

pub mod loader;
mod setup;
mod update;

use self::{
    setup::setup,
    update::handle_input
};

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Controls), setup)
            .init_resource::<ControlsState>()
            .add_systems(
                Update, handle_input.run_if(in_state(AppState::Controls)),
            )
            .add_systems(OnExit(AppState::Controls), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
struct CleanupMarker;
#[derive(Resource)]
pub struct ControlsState {
    pub cooldown: Timer
}

impl Default for ControlsState {
    fn default() -> Self {
        ControlsState {
            cooldown: Timer::from_seconds(0.2, TimerMode::Once)
        }
    }
}

