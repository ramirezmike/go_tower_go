use bevy::prelude::*;
use crate::{assets::GameAssets, cleanup, ui, IngameState, ingame::{player, race, kart, points, game_settings}, util::audio};

pub struct PreGamePlugin;
impl Plugin for PreGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IngameState::PreGame), setup)
            .add_systems(Update, fade_go.run_if(in_state(IngameState::InGame)))
            .add_systems(Update, update_countdown_text.run_if(in_state(IngameState::PreGame)));
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
    mut audio: audio::GameAudio,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
    mut previous: Local<i32>,
) {
    for mut text in &mut texts {
        let value = match game_state.pregame_cooldown.tick(time.delta()).remaining_secs() {
            x if x >= 1.0 => {
                let current = x as i32;
                if current != *previous {
                    audio.play_sfx(&game_assets.sfx_1);
                }
                *previous = current;
                format!("{}", x as i32).to_string()
            },
            _ => {
                audio.play_bgm(&game_assets.bgm_1);
                next_ingame_state.set(IngameState::InGame);
                "GO!".to_string()
            }
        };
        text.sections[0].value = value;
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
        },CleanupMarker))
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
                CleanupMarker,
                CountdownTextMarker,
            ));
        })
        .id();

    commands.entity(root_node).add_child(countdown_container);
}

fn fade_go(
    mut commands: Commands,
    to_clean: Query<Entity, With<CleanupMarker>>,
    mut background_colors: Query<&mut BackgroundColor, With<CleanupMarker>>,
    mut texts: Query<&mut Text, With<CleanupMarker>>,
    time: Res<Time>,
) {
    let mut despawn_entites = true; 
    for mut bg_color in &mut background_colors {
        let a = bg_color.0.a();
        if a > 0.0 {
            despawn_entites = false;
            bg_color.0.set_a(a - (time.delta_seconds() * 1.25));
        }
    }

    for mut text in &mut texts{
        for text_section in text.sections.iter_mut() {

            let a = text_section.style.color.a();
            if a > 0.0 {
                despawn_entites = false;
                text_section.style.color.set_a(a - (time.delta_seconds() * 1.25));
            }
        }
    }

    if despawn_entites {
        for entity in &to_clean {
            commands.entity(entity).despawn_recursive();
        }
    }
}
