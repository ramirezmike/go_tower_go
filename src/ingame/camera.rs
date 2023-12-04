use bevy::{prelude::*, ecs::system::Command};
use smooth_bevy_cameras::{
    controllers::fps::FpsCameraController,
    LookTransformPlugin, 
};
use super::{player, controller};
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::PhysicsSet;
use bevy_xpbd_3d::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookTransformPlugin)
//            .add_systems(Update, follow_player.after(controller::apply_movement_damping));
            .add_systems(
                PostUpdate,
               follow_player 
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

fn follow_player(
    players: Query<(&Transform, &LinearVelocity), With<player::Player>>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<FpsCameraController>, Without<player::Player>)>,
    time: Res<Time>,
//  mut cameras: Query<&mut Transform, >,
) {
    for mut c_transform in &mut cameras {
        for (player_transform, linear_velocity) in &players {

            c_transform.translation = player_transform.translation + (player_transform.back() * 10.0) + Vec3::new(0., 2.7, 0.);
            let between_velocity_and_facing = linear_velocity.0.lerp(player_transform.translation + (player_transform.forward() * 4.0), 0.96);
            let look_at = c_transform.looking_at(between_velocity_and_facing + Vec3::new(0., 0.8, 0.), Vec3::Y);
            c_transform.rotation = c_transform.rotation.slerp(look_at.rotation, 1. -time.delta_seconds());
        }
    }
}

pub struct SpawnCamera<C: Component + Clone> {
    pub cleanup_marker: C
}
impl<C: Component + Clone> Command for SpawnCamera<C> {
    fn apply(self, world: &mut World) {
        world
            .spawn((Camera3dBundle {
                transform: Transform::from_xyz(-5.6, 2.7, 0.)
                    .looking_at(Vec3::new(0., 0.8, 0.), Vec3::Y),
                ..default()
            },self.cleanup_marker));
    }
}
