use bevy::{prelude::*, ecs::system::Command};
use smooth_bevy_cameras::{
    controllers::fps::FpsCameraController,
    LookTransformPlugin, 
};
use super::{player, controller};
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::PhysicsSet;
use bevy_turborand::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_camera_shake::{RandomSource, Shake3d};

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
            let new_translation = player_transform.translation + (player_transform.back() * 20.0) + Vec3::new(0., 8.0, 0.);
            let diff = new_translation - c_transform.translation;
            // try to slow down Y changes
            let y_speed = 2.;
            c_transform.translation += (diff * Vec3::new(1., 0., 1.)) 
                + Vec3::new(0., diff.y * time.delta_seconds() * y_speed, 0.);
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

        let shake_id = world 
            .spawn((Shake3d {
                max_offset: Vec3::new(0.0, 0.0, 0.0),
                max_yaw_pitch_roll: Vec3::new(0.1, 0.1, 0.1),
                trauma: 0.0,
                trauma_power: 2.0,
                decay: 0.8,
                random_sources: [
                    Box::new(RandomShake),
                    Box::new(RandomShake),
                    Box::new(RandomShake),
                    Box::new(RandomShake),
                    Box::new(RandomShake),
                    Box::new(RandomShake),
                ],
            },
            SpatialBundle::default()))
            .id();

        let transform;
        #[cfg(feature = "debug")]
        {
            transform = Transform::from_xyz(-5.6, 2.7, 0.).looking_at(Vec3::new(0., 0.8, 0.), Vec3::Y);
        }
        #[cfg(not(feature = "debug"))]
        {
            transform = Transform::from_xyz(100., 250., -2.9).looking_at(Vec3::new(0., 0.8, 0.), Vec3::Y);
        }

        let camera_id = 
        world
            .spawn((Camera3dBundle {
                transform,
                ..default()
            },self.cleanup_marker, )).id();

        if let Some(mut entity) = world.get_entity_mut(shake_id) {
            entity.push_children(&[camera_id]);
        }
    }
}

fn random_number() -> f32 {
    let rng = Rng::new();
    let x: f32 = rng.f32();
    x * 2.0 - 1.0
}

struct RandomShake;
impl RandomSource for RandomShake {
    fn rand(&self, _time: f32) -> f32 {
        random_number()
    }
}
