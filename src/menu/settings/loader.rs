use crate::{assets::loader::AssetsHandler, assets};
use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};

pub struct SettingsMenuLoader;
impl Command for SettingsMenuLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            AssetsHandler,
            ResMut<assets::GameAssets>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

        assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");
    }
}
