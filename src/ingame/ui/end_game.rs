use bevy::prelude::*;
use crate::{assets::GameAssets, cleanup, ui, IngameState, ingame::{player, race, kart, points, game_settings}, AppState, util::audio};
use crate::assets::command_ext::*;
use std::time::Duration;

pub struct EndGamePlugin;
impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IngameState::EndGame), setup)
        .add_systems(Update, handle_input.run_if(in_state(IngameState::EndGame)))
        .add_systems(OnExit(IngameState::EndGame), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
struct CleanupMarker;

fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    buttons: Res<Input<GamepadButton>>,
    mut cooldown: Local<f32>,
    time: Res<Time>,
) {
    *cooldown += time.delta_seconds();
    *cooldown = cooldown.max(10.);
    if *cooldown > 1. {
        for gamepad in gamepads.iter() {
            if buttons.just_pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::South }) || 
               buttons.just_pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::Start }) {
                commands.load_state(AppState::Splash);
                *cooldown = 0.
            }
        }

        if keyboard_input.any_pressed([KeyCode::Return, KeyCode::Space, ]) {
            *cooldown = 0.;
            commands.load_state(AppState::Splash);
        }
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
    window_size: Res<ui::text_size::WindowSize>,
    text_scaler: ui::text_size::TextScaler,
    game_state: Res<game_settings::GameState>,
    mut audio: audio::GameAudio,
    karts: Query<Entity, With<kart::Kart>>,
) {
    let root_node = 
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(CleanupMarker)
        .id();

    let main_container= 
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(1., 1., 1., 0.7)),
            ..default()
        }).id();

    let top_text_container = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                              match game_state.ending_state {
                                  game_settings::GameEndingState::Winner => "You Won!",
                                  game_settings::GameEndingState::Died => "Knocked Out!",
                                  game_settings::GameEndingState::FellBehind => "Fell Behind!",
                                  _ => "Hey uh.. what happened?? <_<"
                              },
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                },
            ));

        })
        .id();

    let center_text_container =  commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(80.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|builder| {
            let number_of_racers = karts.iter().len() + if game_state.ending_state != game_settings::GameEndingState::Winner { 1 } else { 0 };
            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("Time: {:02}:{:02}", game_state.game_time as isize / 60, game_state.game_time as isize % 60),
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                },
            ));

            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("Place: {} / {}", game_state.player_place, number_of_racers),
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                },
            ));

            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("Laps: {}", game_state.player_lap),
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                },
            ));

            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("Peak Entity Count: {}", game_state.peak_number_of_entities),
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                },
            ));
        })
        .id();

        let menu_button = commands 
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
                                    background_color: ui::HOVERED_BUTTON.into(),
                                    ..Default::default()
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        "MAIN MENU",
                                        TextStyle {
                                            font: game_assets.font.clone(),
                                            font_size: text_scaler.scale(ui::BUTTON_LABEL_FONT_SIZE),
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });
                            });
                }).id();

    commands.entity(main_container).add_child(top_text_container);
    commands.entity(main_container).add_child(center_text_container);
    commands.entity(main_container).add_child(menu_button);
    commands.entity(root_node).add_child(main_container);

    audio.play_bgm(&game_assets.bgm_2);
}
