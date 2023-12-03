use bevy::prelude::*;
use bevy_turborand::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_mod_outline::*;

mod assets;
mod ingame;
mod util;

#[cfg(feature = "debug")]
mod debug;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins((PhysicsPlugins::default(), RngPlugin::default(), OutlinePlugin))
        .add_plugins((assets::AssetsPlugin, util::UtilPlugin, ingame::InGamePlugin, ))
        .add_systems(Update, bootstrap.run_if(in_state(AppState::Initial)))
        .add_state::<AppState>();

    #[cfg(feature = "debug")]
    {
        app.add_plugins(debug::DebugPlugin);
    }

    #[cfg(feature = "inspect")]
    {
        use bevy_inspector_egui::{bevy_egui, quick::WorldInspectorPlugin};
        app.add_plugins(WorldInspectorPlugin::new())
            .insert_resource(bevy_egui::EguiSettings {
                scale_factor: 1.8,
                ..default()
            });
    }

    app.run();
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Initial,
    Loading,
    InGame,
}

use assets::command_ext::*;
fn bootstrap(mut commands: Commands) {
    #[cfg(feature = "debug")]
    {
        commands.load_state(AppState::InGame);
    }

    #[cfg(not(feature = "debug"))]
    commands.load_state(AppState::InGame);
}
