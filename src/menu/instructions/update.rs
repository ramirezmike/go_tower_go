use crate::assets::command_ext::*;
use crate::{assets, util::audio, util::input, AppState};
use super::InstructionsState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn handle_input(
    mut commands: Commands,
    action_state: Query<&ActionState<input::MenuAction>>,
    game_assets: Res<assets::GameAssets>,
    mut audio: audio::GameAudio,
    mut instructions_state: ResMut<InstructionsState>,
    time: Res<Time>,
) {
    let action_state = action_state.single();

    if instructions_state.cooldown.tick(time.delta()).finished() {
        if action_state.just_pressed(input::MenuAction::Select)
            || action_state.just_pressed(input::MenuAction::Start)
        {
            commands.load_state(AppState::Controls);
        }
    }
}
