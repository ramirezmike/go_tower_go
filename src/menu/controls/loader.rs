use crate::{assets::loader::AssetsHandler, assets};
use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};

pub struct ControlsLoader;
impl Command for ControlsLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            AssetsHandler,
            ResMut<assets::GameAssets>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

        assets_handler.add_material(&mut game_assets.controls_gamepad, "textures/controls_gamepad.png", true);
        assets_handler.add_material(&mut game_assets.controls_keyboard, "textures/controls_keyboard.png", true);
    }
}
