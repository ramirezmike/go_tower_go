use super::state::{Settings, SettingsMenuState};
use super::{CleanupMarker, SettingDisplayMarker};
use crate::util::input::InputCommandsExt;
use crate::{assets, menu, menu::MenuOption, ui};
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    text_scaler: ui::text_size::TextScaler,
    mut setting_state: ResMut<SettingsMenuState>,
) {
    *setting_state = SettingsMenuState::default();

    #[cfg(feature = "web")]
    {
        setting_state.enable_shadows = 1;
        setting_state.enable_background = 1;
    }

    #[cfg(not(feature = "web"))]
    {
        setting_state.enable_shadows = 1;
        setting_state.enable_background = 1;
    }

    setting_state.selected_setting = Settings::Go;
    setting_state.screen_cooldown = Timer::from_seconds(0.1, TimerMode::Once);
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

    let root_node = commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(-100),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            },
            CleanupMarker,
        ))
        .id();

    let title_text = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(20.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text::from_section(
                    "Game Settings",
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE * 1.2),
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .id();

    let settings = Settings::get()
        .into_iter()
        .map(|setting| match setting {
            Settings::Go => commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(20.),
                            height: Val::Percent(15.),
                            display: Display::Flex,
                            padding: UiRect::all(Val::Percent(2.)),
                            margin: UiRect {
                                top: Val::Percent(10.),
                                ..default()
                            },
                            align_items: AlignItems::Center,
                            align_self: AlignSelf::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Percent(1.0)),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        ..default()
                    },
                    setting,
                ))
                .with_children(|builder| {
                    builder.spawn((
                        TextBundle {
                            text: Text::from_section(
                                format!("{}", setting.get_label()),
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        },
                        setting,
                    ));
                })
                .id(),
            _ => commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(15.),
                            display: Display::Flex,
                            padding: UiRect::all(Val::Percent(2.)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    setting,
                ))
                .with_children(|builder| {
                    builder.spawn((
                        TextBundle {
                            text: Text::from_section(
                                format!("{}:", setting.get_label()),
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        },
                        setting,
                    ));

                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.),
                                width: Val::Percent(40.),
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "<",
                                        TextStyle {
                                            font: game_assets.font.clone(),
                                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                },
                                setting,
                            ));
                            builder.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "5",
                                        TextStyle {
                                            font: game_assets.font.clone(),
                                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                },
                                setting,
                                SettingDisplayMarker,
                            ));
                            builder.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        ">",
                                        TextStyle {
                                            font: game_assets.font.clone(),
                                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                },
                                setting,
                            ));
                        });
                })
                .id(),
        })
        .collect::<Vec<_>>();

    commands.entity(root_node).add_child(title_text);

    for entity in settings {
        commands.entity(root_node).add_child(entity);
    }
}
