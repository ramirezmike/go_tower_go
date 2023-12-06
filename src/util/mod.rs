use bevy::prelude::*;

pub mod scene_hook;
pub mod screen_shake;

pub struct UtilPlugin;
impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(scene_hook::HookPlugin);
    }
}
