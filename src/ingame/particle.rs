use crate::{assets, ingame};
use bevy::prelude::*;

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_create_particle_event, handle_billboards, animate_particles))
            .add_event::<CreateParticleEvent>();
    }
}

#[derive(Event)]
pub struct CreateParticleEvent {
    pub position: Transform,
}

#[derive(Component)]
pub struct Particle {
    time_to_live: Timer,
    parent: Entity,
    position: Vec3,
}

#[derive(Component)]
pub struct Billboard;

fn handle_billboards(
    mut billboards: Query<&mut Transform, With<Billboard>>,
    camera: Query<&Transform, (With<Camera>, Without<Billboard>)>,
) {
    if let Ok(camera) = camera.get_single() {
        for mut billboard in billboards.iter_mut() {
            billboard.look_at(camera.translation, Vec3::Y);
        }
    }
}

pub fn animate_particles(
    mut commands: Commands,
    mut particles: Query<(
        &mut Particle,
        &mut Transform,
        &Handle<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 3.0;
    for (mut particle, mut transform, material) in
            &mut particles
    {
        if particle.time_to_live.tick(time.delta()).finished() {
            commands.entity(particle.parent).despawn_recursive();
        } else {
            //transform.scale += Vec3::splat(time.delta_seconds() * SPEED);
            transform.scale = transform.scale.clamp_length(0., 2.);
            transform.scale *= 1.0 - (time.delta_seconds() * 0.3);

            if let Some(material) = materials.get_mut(material) {
               let a = material.base_color.a();
               if a > 0.0 {
                   material.base_color.set_a(a - (time.delta_seconds() * 0.25));
               }
            }

            let target = transform
                .translation
                .lerp(Vec3::Y, time.delta_seconds() * 0.3);
            if !target.is_nan() {
                transform.translation = target;
            }
        }
    }
}

pub fn handle_create_particle_event(
    mut commands: Commands,
    mut create_particle_event_reader: EventReader<CreateParticleEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<assets::GameAssets>,
) {
    for event in create_particle_event_reader.read() {
        let material = materials.add(StandardMaterial {
                           base_color_texture: Some(game_assets.smoke_image.image.clone()),
                           alpha_mode: AlphaMode::Blend,
                           ..Default::default()
                       });


        let base_location = event.position.translation + event.position.back().normalize() * 1.2;
        let left = base_location + event.position.left().normalize() * 1.2;
        let right = base_location + event.position.right().normalize() * 1.2;

        for translation in [left, right].iter() {
            let transform = Transform::from_translation(*translation);
            let mut billboard= commands .spawn( ( SpatialBundle::from_transform(transform), Billboard),);
            let billboard_id = billboard.id();
            billboard.with_children(|parent| {
                    parent.spawn((PbrBundle {
                        mesh: game_assets.smoke.clone(),
                        material: material.clone(),
                        transform: Transform::from_rotation(
                            Quat::from_axis_angle(Vec3::X, (3.0 * std::f32::consts::PI) / 2.0)),
                        ..Default::default()
                    },
                    bevy::pbr::NotShadowCaster,
                    Particle {
                        parent: billboard_id,
                        time_to_live: Timer::from_seconds(2., TimerMode::Once),
                        position: transform.translation,
                    },
                    ingame::CleanupMarker,
                    ));
                });
        }
    }
}

