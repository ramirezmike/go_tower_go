use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    app::AppExit,
};
use smooth_bevy_cameras::controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};
use crate::{AppState,};
use crate::ingame::{tower::TowerSpawner, player::Player, };

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (fps_update, debug))
            .add_plugins(FpsCameraPlugin::default())
            .add_plugins((FrameTimeDiagnosticsPlugin::default(),));
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.0,
                color: Color::BLACK,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default() })
        .with_background_color(Color::rgba(1., 1., 1., 0.2)),
        FpsText,
    ));
}

fn fps_update(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn debug(
    mut commands: Commands,
    keys: ResMut<Input<KeyCode>>,
    mut exit: ResMut<Events<AppExit>>,
    mut cameras: Query<(Entity, Option<&mut FpsCameraController>), With<Camera>>,
    player: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
    if keys.just_pressed(KeyCode::R) {
        next_state.set(AppState::Initial);
    }
    if keys.just_pressed(KeyCode::C) {
        for (camera, maybe_fps) in &mut cameras {
            match maybe_fps {
                Some(mut fps) => {
                    fps.enabled = !fps.enabled;
                },
                None => { 
                    commands.entity(camera)
                                .insert(FpsCameraBundle::new(
                                    FpsCameraController {
                                        enabled: true,
                                        translate_sensitivity: 20.0,
                                        ..default()
                                    },
                                    //Vec3::new(-31.0, 244.0, 47.0),
                                    Vec3::new(0.0, 5.0, 0.0),
                                    Vec3::new(-31., 0., 47.),
                                    Vec3::Y,
                                ));
                },
                _ => ()
            };
        }
    }
    if keys.just_pressed(KeyCode::V) {
        for (camera, maybe_fps) in &mut cameras {
            match maybe_fps {
                Some(_) => {
                    commands.entity(camera).remove::<FpsCameraController>()
                    .remove::<Smoother>()
                    .remove::<LookTransform>();
                },
                _ => ()
            };
        }
    }
//  if keys.just_pressed(KeyCode::T) {
//      commands.add(TowerSpawner {
//          entity: player.single()
//      });
//  }
}
