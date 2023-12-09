use crate::{
    assets::GameAssets, util::input::InputCommandsExt, util::audio, cleanup, menu::MenuOption, ui, AppState,
};
use bevy::prelude::*;

pub mod loader;
mod state;
mod update;

use self::{
    state::{TitleScreenOptions, TitleScreenState},
    update::{handle_input, highlight_selection},
};

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::TitleScreen), setup)
            .init_resource::<TitleScreenState>()
            .add_systems(
                Update,
                (highlight_selection, handle_input).run_if(in_state(AppState::TitleScreen)),
            )
            .add_systems(OnExit(AppState::TitleScreen), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
pub struct CleanupMarker;

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut audio: audio::GameAudio,
    text_scaler: ui::text_size::TextScaler,
) {
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

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            },
            CleanupMarker,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Percent(80.0),
                    margin: UiRect {
                        top: Val::Percent(2.5),
                        ..default()
                    },
                    ..default()
                },
                image: game_assets.title_screen_logo.image.clone().into(),
                ..default()
            });
        });

    commands.spawn((
        TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },
            text: Text::from_section(
                "by michael ramirez".to_string(),
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: text_scaler.scale(ui::BY_LINE_FONT_SIZE),
                    color: Color::rgba(0.0, 0.0, 0.0, 1.0),
                },
            ),
            ..default()
        },
        CleanupMarker,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(98.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            },
            CleanupMarker,
        ))
        .with_children(|parent| {
            TitleScreenOptions::get().into_iter().for_each(|option| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                width: Val::Percent(18.0),
                                height: Val::Percent(12.5),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: ui::NORMAL_BUTTON.into(),
                            ..Default::default()
                        },
                        option,
                    ))
                    .with_children(|parent| {
                        parent.spawn((TextBundle {
                            text: Text::from_section(
                                option.get_label(),
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size: text_scaler.scale(ui::BUTTON_LABEL_FONT_SIZE),
                                    color: Color::rgb(0.0, 0.0, 0.0),
                                },
                            ),
                            ..default()
                        }, option));
                    });
            });
        });
}
