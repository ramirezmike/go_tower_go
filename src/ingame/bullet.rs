use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use std::f32::consts::TAU;
use bevy_xpbd_3d::prelude::*;
use bevy_turborand::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use crate::{assets, ingame, ingame::{config, collisions}, util, AppState};
use bevy_kira_audio::prelude::*;

use super::game_settings;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_create_hit_event, animate_hit, update_bullets, ).run_if(in_state(AppState::InGame)))
            .add_systems(
                FixedUpdate,
                (handle_extra_entities).run_if(in_state(AppState::InGame)),

            )
            .add_event::<CreateHitEvent>();
    }
}

#[derive(Component)]
pub struct BulletHit {
    pub move_toward: Vec3,
    pub with_physics: bool,
}

fn kill_time_to_lives( 
    mut commands: Commands,
    mut entities: Query<(Entity, &mut TimeToLive)>,
    time: Res<Time>,
) {
    for (e, mut time_to_live) in &mut entities {
        if time_to_live.0.tick(time.delta()).finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct TimeToLive(Timer);

#[derive(Component)]
pub struct ExtraEntity {
    cooldown: Timer,
    time_to_live: Timer,
}


fn handle_extra_entities(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut ExtraEntity)>,
    time: Res<Time>,
) {
    for (entity, mut extra) in &mut entities {
        if extra.cooldown.tick(time.delta()).finished() {
//          commands.entity(entity).insert(
//              CollisionLayers::new([collisions::Layer::Bullet], [collisions::Layer::Ground, collisions::Layer::Kart]),
//          );

            commands.entity(entity)
                .remove::<RigidBody>()
                .remove::<LinearVelocity>()
                .remove::<CollisionLayers>()
                .remove::<Collider>();
        } 

        if extra.time_to_live.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}


#[derive(Event)]
pub struct CreateHitEvent {
    pub position: Vec3,
    pub count: usize,
    pub color: Color,
    pub material: Handle<StandardMaterial>,
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
    audio: Res<Audio>,
    game_assets: Res<assets::GameAssets>,
    game_state: Res<game_settings::GameState>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for event in create_hit_event_reader.read() {
        let position = event.position;

        let transform =
            Transform::from_xyz(position.x as f32, position.y as f32, position.z as f32);

        for i in 0..event.count {
            let mut emitter = AudioEmitter { instances: vec!(), };
            if i == 0 {
                let sound = audio.play(game_assets.sfx_hit.clone()).with_volume(0.).handle();
                emitter.instances.push(sound);
            }
            let inner_mesh_x = (global_rng.f32_normalized() * 25.) / 100.0;
            let inner_mesh_z = (global_rng.f32_normalized() * 25.) / 100.0;


            let move_toward_x = global_rng.f32_normalized();
            let move_toward_y = global_rng.f32();
            let move_toward_z = global_rng.f32_normalized();
            let move_toward = Vec3::new(move_toward_x, move_toward_y, move_toward_z);

            let material = 
                if game_state.enable_extra_entities {
                    event.material.clone_weak()
                } else {
                    let mut material: StandardMaterial = event.color.into();
                    material.alpha_mode = AlphaMode::Blend;
                    materials.add(material)
                };

            let mut particle = 
            commands
                .spawn((PbrBundle {
                    transform,
                    ..Default::default()
                }, emitter, ingame::CleanupMarker));
            particle
                .with_children(|parent| {
                    let mut entity_cmds = 
                    parent
                        .spawn(PbrBundle {
                            mesh: game_assets.hit_particle.clone_weak(),
                            material,
                            transform: {
                                let mut t = Transform::from_xyz(inner_mesh_x, 0.1, inner_mesh_z);
                                t.rotate(Quat::from_rotation_x(inner_mesh_z));
                                t.rotate(Quat::from_rotation_y(inner_mesh_x));
                                t
                            },
                            visibility: Visibility::Visible,
                            ..Default::default()
                        });
                    if !game_state.enable_extra_entities {
                        entity_cmds.insert(BulletHit { with_physics: event.with_physics, move_toward });
                    }
                });

            if event.with_physics {
                particle.insert((
                    RigidBody::Dynamic,
                    LinearVelocity(move_toward.normalize() * 10.0),
                    CollisionLayers::new([collisions::Layer::Bullet], [collisions::Layer::Ground]),
                    Collider::cuboid(0.1, 0.1, 0.1),
                ));
            }

            if game_state.enable_extra_entities {
                particle.insert((
                    ExtraEntity {
                        cooldown: Timer::from_seconds(4., TimerMode::Once),
                        time_to_live: Timer::from_seconds(config::SHRAPNEL_TIME_TO_LIVE, TimerMode::Once),
                    }
                ));
            }
        }
    }
}

pub struct BulletSpawner<C: Component + Clone>  {
    pub spawn_point: Vec3,
    pub direction: Vec3,
    pub material: Handle<StandardMaterial>,
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
            Res<Audio>,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, assets_gltf, audio) = system_state.get_mut(world);
        let mesh = assets_handler.meshes.add(shape::UVSphere { radius: 1.0, sectors: 3, stacks: 6 }.into()).clone();
        let material = assets_handler.materials.add(self.color.into()).clone(); 

        world.spawn((
            PbrBundle {
                mesh, 
                material,
                transform: Transform::from_translation(self.spawn_point).with_rotation(Quat::from_axis_angle(Vec3::Y, TAU * 0.75)),
                ..default()
            },
//          AudioEmitter {
//              instances: vec![sfx],
//          },
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
                material: self.material.clone_weak(),
                direction: self.direction,
                color: self.color,
                speed: self.speed,
            },
            self.cleanup_marker,
        )).with_children(|builder| {
//          builder.spawn(PointLightBundle {
//              point_light: PointLight {
//                  intensity: 1600.0,
//                  color: self.color,
//                  range: 8.0,
//                  shadows_enabled: true,
//                  ..default()
//              },
//             ..default()
//          });
        });
    }
}

#[derive(Component)]
pub struct Bullet {
    pub owner: Entity,
    pub direction: Vec3,
    pub speed: f32,
    pub color: Color,
    pub material: Handle<StandardMaterial>
}

fn update_bullets(
    mut bullets: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}
