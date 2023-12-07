use bevy::prelude::*;
use crate::{assets::GameAssets, cleanup, ui, IngameState, ingame::{player, race, kart, points, game_settings}};

pub struct EndGamePlugin;
impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IngameState::EndGame), setup)
        .add_systems(OnExit(IngameState::EndGame), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
struct CleanupMarker;

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
    window_size: Res<ui::text_size::WindowSize>,
    text_scaler: ui::text_size::TextScaler,
    game_state: Res<game_settings::GameState>,
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
                flex_direction: FlexDirection::Row,
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
                              match game_state.is_winner {
                                  true => "You Won!",
                                  false => "You Lost!",
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

    commands.entity(main_container).add_child(top_text_container);
    commands.entity(root_node).add_child(main_container);
}
