use bevy::prelude::*;
use crate::{assets::GameAssets, cleanup, ui, IngameState, ingame::{player, race, kart, points, game_settings}};

pub struct PreGamePlugin;
impl Plugin for PreGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IngameState::PreGame), setup)
        .add_systems(Update, update_countdown_text.run_if(in_state(IngameState::PreGame)))
        .add_systems(OnExit(IngameState::PreGame), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Component)]
struct CountdownTextMarker;

fn update_countdown_text(
    mut game_state: ResMut<game_settings::GameState>,
    mut texts: Query<&mut Text, With<CountdownTextMarker>>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    time: Res<Time>,
) {
    for mut text in &mut texts {
        let value = match game_state.pregame_cooldown.tick(time.delta()).remaining_secs() {
            x if x >= 1.0 => format!("{}", x as i32).to_string(),
            _ => "GO!".to_string()
        };
        text.sections[0].value = value;
    }

    if game_state.pregame_cooldown.finished() {
        next_ingame_state.set(IngameState::InGame);
    }
}

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

    let countdown_container = commands
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
            background_color: BackgroundColor(Color::rgba(1., 1., 1., 0.7)),
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE * 3.),
                            color: Color::DARK_GRAY,
                        },
                    ),
                    ..default()
                },
                CountdownTextMarker,
            ));
        })
        .id();

    commands.entity(root_node).add_child(countdown_container);
}
