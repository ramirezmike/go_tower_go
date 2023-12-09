use bevy::prelude::*;

pub mod splash;
pub mod controls;
pub mod instructions;
pub mod title_screen;
pub mod settings;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            splash::SplashPlugin,
            title_screen::TitlePlugin,
            controls::ControlsPlugin,
            instructions::InstructionsPlugin,
            settings::SettingsMenuPlugin,
        ));
    }
}

trait MenuOption<const N: usize>
where
    Self: PartialEq + Sized + Clone + Copy,
{
    const ITEM: [Self; N];

    fn get_label(&self) -> &str;

    fn get() -> [Self; N] {
        Self::ITEM
    }

    fn next(&self) -> Self {
        let position = Self::ITEM.iter().position(|x| x == self).unwrap();
        *Self::ITEM.iter().cycle().nth(position + 1).unwrap()
    }

    fn previous(&self) -> Self {
        let position = Self::ITEM.iter().rev().position(|x| x == self).unwrap();
        *Self::ITEM.iter().rev().cycle().nth(position + 1).unwrap()
    }
}
