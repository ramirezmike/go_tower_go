use crate::{assets::GameAssets, cleanup, ui, IngameState, ingame::{player, race, kart}};
use bevy::prelude::*;
use std::collections::HashMap;

const UI_UPDATE: f64 = 0.5;
pub struct InGameUIPlugin;
impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IngameState::InGame), setup)
        .insert_resource(Time::from_seconds(UI_UPDATE))
        .add_systems(
            FixedUpdate,
            (update_lap_counter, update_place).run_if(in_state(IngameState::InGame)),
        )
        .add_systems(OnExit(IngameState::InGame), cleanup::<CleanupMarker>);
    }
}

#[derive(Component, Clone)]
struct CleanupMarker;

#[derive(Component)]
struct LapMarker;

#[derive(Component)]
struct PlaceMarker;

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
    window_size: Res<ui::text_size::WindowSize>,
    text_scaler: ui::text_size::TextScaler,
) {
    let root_node = 
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .insert(CleanupMarker)
        .id();

    let top_row = 
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(10.0),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        }).id();

    let top_row_left_side = 
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Auto,
                position_type: PositionType::Relative,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        }).id();

    let top_row_right_side= 
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Auto,
                position_type: PositionType::Relative,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        }).id();

    let lap_counter_node = 
       commands 
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            }).id();

    let lap_counter =
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    "Lap 1",
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },
            LapMarker,
        )).id();

    let place_node= 
       commands 
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            }).id();

    let place=
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },
            PlaceMarker, // haha
        )).id();

    commands.entity(place_node).add_child(place);
    commands.entity(lap_counter_node).add_child(lap_counter);
    commands.entity(top_row_left_side).add_child(lap_counter_node);
    commands.entity(top_row_right_side).add_child(place_node);
    commands.entity(top_row).add_child(top_row_left_side);
    commands.entity(top_row).add_child(top_row_right_side);
    commands.entity(root_node).add_child(top_row);
}

fn update_place(
    player_place: Query<&race::placement_sensor::Place, With<player::Player>>,
    total_racers: Query<Entity, With<kart::Kart>>,
    mut texts: Query<&mut Text, With<PlaceMarker>>,
) {
    for mut text in &mut texts {
        for place in &player_place {
            text.sections[0].value = format!("Place: {} / {}", place.0, total_racers.iter().len());
        }
    }
}

fn update_lap_counter(
    player_lap: Query<&race::LapCounter, With<player::Player>>,
    mut texts: Query<&mut Text, With<LapMarker>>,
) {
    for mut text in &mut texts {
        for lap in &player_lap {
            text.sections[0].value = format!("Lap {}", lap.0);
        }
    }
}
