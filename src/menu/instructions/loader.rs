use crate::{assets::loader::AssetsHandler, assets};
use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};

pub struct InstructionsLoader;
impl Command for InstructionsLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            AssetsHandler,
            ResMut<assets::GameAssets>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

        assets_handler.add_material(&mut game_assets.instructions, "textures/instructions.png", true);
    }
}
