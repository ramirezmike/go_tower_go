use bevy::{prelude::*, ecs::system::{Command, SystemState}, gltf::Gltf, };
use bevy_xpbd_3d::{math::*, prelude::*};
use bevy_turborand::prelude::*;
use std::f32::consts::TAU;
use crate::{assets, AppState, util, IngameState, cleanup};
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};

mod bot;
mod bullet;
mod camera; 
mod kart;
mod controller;
mod collisions;
mod path;
mod race;
mod finish_line;
mod ui;
pub mod player;
pub mod tower;
pub mod config;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, controller::CharacterControllerPlugin, tower::TowerPlugin, bullet::BulletPlugin, bot::BotPlugin, path::PathPlugin, finish_line::FinishLinePlugin, race::RacePlugin, collisions::CollisionsPlugin, ui::InGameUIPlugin,))
            .add_systems(OnExit(AppState::InGame), cleanup::<CleanupMarker>)
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
    game_assets: Res<assets::GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
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

                        if name.contains("delete") {
                            let entity = cmds.id();
                            cmds.commands().entity(entity).despawn_recursive();
                        }

                        if name.contains("path") {
                            if let (Some(global_transform), Some(aabb)) = (hook_data.global_transform, hook_data.aabb) {
                                cmds.commands().add(path::PathAdder { global_transform: *global_transform, aabb: *aabb, name: name.to_string() });
                            }

                            cmds.insert(Visibility::Hidden);
                        }

                        if name.contains("kart_spawner") {
                            if let (Some(global_transform), Some(aabb)) = (hook_data.global_transform, hook_data.aabb) {
                                cmds.commands().add(kart::KartSpawner { global_transform: *global_transform, aabb: *aabb, cleanup_marker: CleanupMarker });
                            }

                            let entity = cmds.id();
                            cmds.commands().entity(entity).despawn_recursive();
                        }

                        if name.contains("waypoint") {
                            let entity = cmds.id();
                            cmds.commands().add(
                                race::WayPointSpawner {
                                    entity,
                                    name: name.to_string(),
                                    mesh: mesh.clone(),
                                }
                            );
                        }

                        if name.contains("place_sensor") {
                            cmds.insert(
                                (race::placement_sensor::PlaceSensor::new(name), 
                                Visibility::Hidden,
                                Collider::trimesh_from_mesh(mesh).unwrap(), 
                            ));
                        }
                    }
                })
            }, 
            CleanupMarker,
        ));
    }

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
    next_ingame_state.set(IngameState::InGame);
}
