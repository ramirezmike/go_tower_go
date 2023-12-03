use bevy::{prelude::*, ecs::system::Command};
use smooth_bevy_cameras::{
    controllers::fps::FpsCameraController,
    LookTransform, LookTransformBundle, LookTransformPlugin, Smoother,
};
use super::player;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookTransformPlugin)
            .add_systems(Update, follow_player);
    }
}

fn follow_player(
    players: Query<&Transform, With<player::Player>>,
    mut cameras: Query<&mut LookTransform, Without<FpsCameraController>>,
) {
    for mut camera_transform in &mut cameras {
        for player_transform in &players {
            camera_transform.eye = player_transform.translation + (player_transform.back() * 10.0) + Vec3::new(0., 2.7, 0.);
            camera_transform.target = player_transform.translation + (player_transform.forward() * 4.0) + Vec3::new(0., 0.8, 0.);
        }
    }
}

pub struct SpawnCamera<C: Component + Clone> {
    pub cleanup_marker: C
}
impl<C: Component + Clone> Command for SpawnCamera<C> {
    fn apply(self, world: &mut World) {
        world
            .spawn(Camera3dBundle::default())
            .insert((
                LookTransformBundle {
                    transform: LookTransform::new(Vec3::new(-5.6, 2.7, 0.), Vec3::new(0., 0.8, 0.), Vec3::Y),
                    smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
                },
                self.cleanup_marker
            ));
    }
}
