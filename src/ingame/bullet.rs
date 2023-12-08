use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use std::f32::consts::TAU;
use bevy_xpbd_3d::prelude::*;
use bevy_turborand::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use crate::{assets, ingame::{config, collisions}, util, AppState};

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_create_hit_event, animate_hit, update_bullets, ).run_if(in_state(AppState::InGame)))
            .add_event::<CreateHitEvent>();
    }
}

#[derive(Component)]
pub struct BulletHit {
    pub move_toward: Vec3,
    pub with_physics: bool,
}

#[derive(Event)]
pub struct CreateHitEvent {
    pub position: Vec3,
    pub count: usize,
    pub color: Color,
    pub with_physics: bool,
}

pub fn animate_hit(
    mut commands: Commands,
    mut hits: Query<(&BulletHit, &mut Transform, &Handle<StandardMaterial>, &Parent)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (hit, mut transform, material, parent) in hits.iter_mut() {
//      transform.rotate(Quat::from_rotation_x(time.delta_seconds()));
//      transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
        transform.scale *= 1.0 - (time.delta_seconds() * config::HIT_SHRINK_SPEED);

        if !hit.with_physics {
            let target = transform
                .translation
                .lerp(hit.move_toward, time.delta_seconds() * config::HIT_SPEED);
            if !target.is_nan() {
                transform.translation = target;
            }
        }

        let mut despawn_entity = true; // if the material doesn't exist, just despawn
        if let Some(material) = materials.get_mut(material) {
            let a = material.base_color.a();
            if a > 0.0 {
                despawn_entity = false;
                material.base_color.set_a(a - (time.delta_seconds() * 1.25));
            }
        }

        if despawn_entity {
            commands.entity(**parent).despawn_recursive();
        }
    }
}

pub fn handle_create_hit_event(
    mut commands: Commands,
    mut create_hit_event_reader: EventReader<CreateHitEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for event in create_hit_event_reader.read() {
        let position = event.position;

        let transform =
            Transform::from_xyz(position.x as f32, position.y as f32, position.z as f32);

        for _ in 0..6 {
            let inner_mesh_x = (global_rng.f32_normalized() * 25.) / 100.0;
            let inner_mesh_z = (global_rng.f32_normalized() * 25.) / 100.0;


            let move_toward_x = global_rng.f32_normalized();
            let move_toward_y = global_rng.f32();
            let move_toward_z = global_rng.f32_normalized();
            let move_toward = Vec3::new(move_toward_x, move_toward_y, move_toward_z);

            let mut material: StandardMaterial = event.color.into();
            material.alpha_mode = AlphaMode::Blend;
            let mut particle = 
            commands
                .spawn(PbrBundle {
                    transform,
                    ..Default::default()
                });
            particle
                .with_children(|parent| {
                    parent
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube {
                                size: 0.25,
                            })),
                            material: materials.add(material),
                            transform: {
                                let mut t = Transform::from_xyz(inner_mesh_x, 0.1, inner_mesh_z);
                                t.rotate(Quat::from_rotation_x(inner_mesh_z));
                                t.rotate(Quat::from_rotation_y(inner_mesh_x));
                                t
                            },
                            visibility: Visibility::Visible,
                            ..Default::default()
                        })
                        .insert(BulletHit { with_physics: event.with_physics, move_toward });
                });

            if event.with_physics {
                particle.insert((
                    RigidBody::Dynamic,
                    LinearVelocity(move_toward.normalize() * 10.0),
                    CollisionLayers::new([collisions::Layer::Bullet], [collisions::Layer::Ground]),
                    Collider::cuboid(0.1, 0.1, 0.1),
                ));
            }
        }
    }
}

pub struct BulletSpawner<C: Component + Clone>  {
    pub spawn_point: Vec3,
    pub direction: Vec3,
    pub owner: Entity,
    pub color: Color,
    pub speed: f32,
    pub cleanup_marker: C
}
impl<C: Component + Clone>  Command for BulletSpawner<C> {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            Res<assets::GameAssets>,
            Res<Assets<Gltf>>,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, assets_gltf) = system_state.get_mut(world);
        let mesh = assets_handler.meshes.add(shape::UVSphere { radius: 1.0, sectors: 3, stacks: 6 }.into()).clone();
        let material = assets_handler.materials.add(self.color.into()).clone(); 

        world.spawn((
            PbrBundle {
                mesh, 
                material,
                transform: Transform::from_translation(self.spawn_point).with_rotation(Quat::from_axis_angle(Vec3::Y, TAU * 0.75)),
                ..default()
            },
            OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    width: 1.0,
                    colour: Color::BLACK,
                },
                mode: OutlineMode::RealVertex,
                ..default()
            },
            Collider::cuboid(1.5, 1.5, 1.5),
//            CollisionLayers::new([collisions::Layer::Bullet], [collisions::Layer::Ground]),
            Bullet {
                owner: self.owner,
                direction: self.direction,
                color: self.color,
                speed: self.speed,
            },
            self.cleanup_marker,
        ));
    }
}

#[derive(Component)]
pub struct Bullet {
    pub owner: Entity,
    pub direction: Vec3,
    pub speed: f32,
    pub color: Color,
}

fn update_bullets(
    mut bullets: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}
