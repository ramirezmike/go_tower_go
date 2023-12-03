use bevy::{prelude::*, ecs::system::{Command, SystemState}, gltf::Gltf, };
use bevy_xpbd_3d::{math::*, prelude::*};
use bevy_turborand::prelude::*;
use std::f32::consts::TAU;
use crate::{assets, AppState, util, };
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};

mod bot;
mod bullet;
mod camera; 
mod car;
mod controller;
pub mod player;
pub mod tower;
pub mod config;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, controller::CharacterControllerPlugin, tower::TowerPlugin, bullet::BulletPlugin, bot::BotPlugin))
            .add_systems(OnEnter(AppState::InGame), setup);

        if cfg!(feature = "colliders") {
            app.add_plugins(PhysicsDebugPlugin::default());
        }
    }
}

pub struct IngameLoader;
impl Command for IngameLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            ResMut<assets::GameAssets>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

        assets_handler.add_glb(&mut game_assets.track, "models/track.glb");
        assets_handler.add_glb(&mut game_assets.tower_01, "models/tower.glb");
    }
}

#[derive(Component, Clone)]
pub struct CleanupMarker;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
    game_assets: Res<assets::GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if let Some(gltf) = assets_gltf.get(&game_assets.track) {
        commands.spawn((
            util::scene_hook::HookedSceneBundle {
                scene: SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                },
                hook: util::scene_hook::SceneHook::new(move |cmds, hook_data| {
                    if let (Some(mesh), Some(name)) = (hook_data.mesh, hook_data.name) {
                        if name.contains("collide") {
                            cmds.insert((
                                RigidBody::Static,
                                Collider::trimesh_from_mesh(mesh).unwrap(), 
                                OutlineBundle {
                                    outline: OutlineVolume {
                                        visible: true,
                                        width: 1.0,
                                        colour: Color::BLACK,
                                    },
                                    mode: OutlineMode::RealVertex,
                                    ..default()
                                },
                            ));
                        }
                    }
                })
            }, 
        ));
    }

    // player placeholder
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0., 0.5, 0.).with_rotation(Quat::from_axis_angle(Vec3::Y, TAU * 0.75)),
            ..default()
        },
        player::Player,
        car::Car,
        OutlineBundle {
            outline: OutlineVolume {
                visible: true,
                width: 1.0,
                colour: Color::BLACK,
            },
            mode: OutlineMode::RealVertex,
            ..default()
        },
        controller::CharacterControllerKeyboard,
        //controller::CharacterControllerBundle::new(Collider::cuboid(1.0, 1.0, 1.0), Vector::NEG_Y * 9.81 * 2.0)
        controller::CommonControllerBundle::new(Collider::capsule(0.3, 0.4), Vector::NEG_Y * 9.81 * 2.0)
    ));

    // bot placeholder
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(1., 0.5, 0.).with_rotation(Quat::from_axis_angle(Vec3::Y, TAU * 0.75)),
            ..default()
        },
        car::Car,
        OutlineBundle {
            outline: OutlineVolume {
                visible: true,
                width: 1.0,
                colour: Color::RED,
            },
            mode: OutlineMode::RealVertex,
            ..default()
        },
        bot::Bot,
        controller::CommonControllerBundle::new(Collider::capsule(0.3, 0.4), Vector::NEG_Y * 9.81 * 2.0)
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_axis_angle(
                Vec3::new(-0.8263363, -0.53950554, -0.16156079),
                2.465743,
            )),
            directional_light: DirectionalLight {
                illuminance: 100000.0,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
        CleanupMarker,
    ));

    commands.add(camera::SpawnCamera { cleanup_marker: CleanupMarker });
}
