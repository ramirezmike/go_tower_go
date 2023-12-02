use bevy::{prelude::*, ecs::system::Command};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookTransformPlugin)
            .add_plugins(FpsCameraPlugin::default());
    }
}

pub struct SpawnCamera<C: Component + Clone> {
    pub cleanup_marker: C
}
impl<C: Component + Clone> Command for SpawnCamera<C> {
    fn apply(self, world: &mut World) {
        world
            .spawn(Camera3dBundle::default())
            .insert((FpsCameraBundle::new(
                FpsCameraController {
                    enabled: false,
                    translate_sensitivity: 20.0,
                    ..default()
                },
                Vec3::new(-9.5, 7.5, 15.3),
                Vec3::new(0., 0., 0.),
                Vec3::Y,
            ), self.cleanup_marker));
    }
}
