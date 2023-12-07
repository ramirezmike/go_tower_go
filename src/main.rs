use bevy::prelude::*;
use bevy_turborand::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_mod_outline::*;
use bevy_camera_shake::CameraShakePlugin;
use bevy::asset::AssetMetaCheck;

mod assets;
mod ingame;
mod util;
mod ui;

#[cfg(feature = "debug")]
mod debug;

fn main() {
    let mut app = App::new();
    app.insert_resource(AssetMetaCheck::Never);

    #[cfg(not(feature = "web"))]
    {
       app.add_plugins(DefaultPlugins);
    }

    #[cfg(feature = "web")]
    {
        app.add_plugins(DefaultPlugins.set(AssetPlugin {
          ..default()
        })
         .set(WindowPlugin {
          primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..default()
          }),
          ..default()
        }));
    }

    app
        .add_plugins((PhysicsPlugins::default(), RngPlugin::default(), OutlinePlugin, CameraShakePlugin,))
        .add_plugins((assets::AssetsPlugin, util::UtilPlugin, ingame::InGamePlugin, 
            ui::text_size::TextSizePlugin,
            ui::follow_text::FollowTextPlugin,
        ))
        .add_systems(Update, bootstrap.run_if(in_state(AppState::Initial)))
        .add_state::<IngameState>()
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

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum IngameState {
    InGame,
    EndGame,
    PreGame, // haha yeaaah
    #[default]
    Disabled,
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

pub fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
