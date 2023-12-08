use crate::util::num_ext::*;
use crate::{menu::MenuOption};
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SettingsMenuState {
    pub screen_cooldown: Timer,
    pub selected_setting: Settings,
    pub enable_shadows: isize,
    pub enable_background: isize,
    pub enable_extra_physics: isize,
    pub enable_extra_entities: isize,
}

impl SettingsMenuState {
    pub fn display(&self, setting: &Settings) -> String {
        match setting {
            Settings::EnableBackground => match self.enable_background {
                1 => "     On     ".to_string(),
                _ => "     Off    ".to_string(),
            },
            Settings::EnableExtraPhysics => match self.enable_extra_physics {
                1 => "     On     ".to_string(),
                _ => "     Off    ".to_string(),
            },
            Settings::EnableExtraEntities => match self.enable_extra_entities {
                1 => "     On     ".to_string(),
                _ => "     Off    ".to_string(),
            },
            Settings::EnableShadows => match self.enable_shadows {
                1 => "     On     ".to_string(),
                _ => "     Off    ".to_string(),
            },
            setting => setting.get_label().to_string(),
        }
    }

    pub fn increment(&mut self) {
        match self.selected_setting {
            Settings::EnableShadows => {
                self.enable_shadows = self.enable_shadows.circular_increment(0, 1);
            },
            Settings::EnableBackground  => {
                self.enable_background = self.enable_background.circular_increment(0, 1);
            },
            Settings::EnableExtraEntities => {
                self.enable_extra_entities = self.enable_extra_entities.circular_increment(0, 1);
            },
            Settings::EnableExtraPhysics => {
                self.enable_extra_physics = self.enable_extra_physics.circular_increment(0, 1);
            },
            _ => (),
        }
    }

    pub fn decrement(&mut self) {
        match self.selected_setting {
            Settings::EnableShadows => {
                self.enable_shadows = self.enable_shadows.circular_decrement(0, 1);
            },
            Settings::EnableBackground  => {
                self.enable_background = self.enable_background.circular_decrement(0, 1);
            },
            Settings::EnableExtraEntities => {
                self.enable_extra_entities = self.enable_extra_entities.circular_decrement(0, 1);
            },
            Settings::EnableExtraPhysics => {
                self.enable_extra_physics = self.enable_extra_physics.circular_decrement(0, 1);
            },
            _ => (),
        }
    }
}

#[derive(Component, Copy, Clone, PartialEq, Default)]
pub enum Settings {
    #[default]
    EnableShadows,
    EnableBackground,
    EnableExtraPhysics,
    EnableExtraEntities, 
    Go,
}

impl MenuOption<5> for Settings {
    const ITEM: [Settings; 5] = [
        Settings::EnableShadows,
        Settings::EnableBackground,
        Settings::EnableExtraPhysics,
        Settings::EnableExtraEntities,
        Settings::Go,
    ];

    fn get_label(&self) -> &str {
        match self {
            Settings::EnableShadows => "Shadows",
            Settings::EnableBackground => "Background",
            Settings::EnableExtraPhysics => "Extra Physics",
            Settings::EnableExtraEntities => "Extra Entities",
            Settings::Go => "Go!",
        }
    }
}
