use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    app::AppExit,
};
use smooth_bevy_cameras::controllers::fps::FpsCameraController;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraController;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (fps_update, debug))
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
    mut cameras: Query<(Entity, Option<&mut FpsCameraController>, Option<&mut OrbitCameraController>)>,
) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
    if keys.just_pressed(KeyCode::C) {
        for (camera, maybe_fps, maybe_orbit) in &mut cameras {
            if maybe_fps.is_some() {
                let mut controller = maybe_fps.unwrap();
                controller.enabled = !controller.enabled;
            }
            if maybe_orbit.is_some() {
                let mut controller = maybe_orbit.unwrap();
                controller.enabled = !controller.enabled;
            }
        }
    }
    if keys.just_pressed(KeyCode::V) {
        for (camera, maybe_fps, maybe_orbit) in &mut cameras {
            if maybe_fps.is_some() {
                commands.entity(camera).remove::<FpsCameraController>();
                commands.entity(camera).insert(OrbitCameraController::default());
            }
            if maybe_orbit.is_some() {
                commands.entity(camera).remove::<OrbitCameraController>();
                commands.entity(camera).insert(FpsCameraController {
                    enabled: true,
                    translate_sensitivity: 20.0,
                    ..default()
                });
            }
        }
    }
}
