use super::{CleanupMarker, ControlsState};
use crate::ingame::game_settings;
use crate::util::input::InputCommandsExt;
use crate::assets;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    game_state: Res<game_settings::GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut controls_state: ResMut<ControlsState>,
) {
    let camera_transform = Transform::from_xyz(0.0, 4., 0.0).looking_at(Vec3::ZERO, -Vec3::Z);
    commands.spawn((
        Camera3dBundle {
            camera: Camera { ..default() },
            ..default()
        },
        CleanupMarker,
        ViewVisibility::default(),
        Visibility::Visible,
    ));
    commands.spawn_menu_input(CleanupMarker);
    *controls_state = ControlsState::default();

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    ..default()
                },
                image: match game_state.controller_type {
                    game_settings::ControllerType::Gamepad => game_assets.controls_gamepad.image.clone().into(),
                    _ => game_assets.controls_keyboard.image.clone().into(),
                },
                ..default()
            });
        });
}
