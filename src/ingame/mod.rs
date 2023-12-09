use bevy::{prelude::*, ecs::system::{Command, SystemState}, gltf::Gltf, };
use bevy_xpbd_3d::{math::*, prelude::*};
use bevy_turborand::prelude::*;
use std::f32::consts::TAU;
use crate::{assets, AppState, util, IngameState, cleanup, shaders};
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use bevy_kira_audio::prelude::*;

mod bot;
mod bullet;
pub mod camera; 
mod common;
mod kart;
mod controller;
mod collisions;
mod path;
mod race;
mod finish_line;
pub mod game_settings;
mod points;
mod particle;
mod ui;
pub mod player;
pub mod tower;
pub mod config;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, controller::CharacterControllerPlugin, tower::TowerPlugin, bullet::BulletPlugin, bot::BotPlugin, path::PathPlugin, finish_line::FinishLinePlugin, race::RacePlugin, collisions::CollisionsPlugin, ui::InGameUIPlugin, kart::KartPlugin, particle::ParticlePlugin, common::CommonPlugin,))
            .init_resource::<game_settings::GameState>()
            .add_systems(Update, game_settings::update_game_state.run_if(in_state(IngameState::InGame)))
            .add_systems(OnExit(AppState::InGame), cleanup::<CleanupMarker>)
            .add_systems(OnExit(IngameState::InGame), stop_audio)
            .add_systems(OnEnter(AppState::InGame), stop_audio)
            .add_systems(OnExit(AppState::Controls), stop_audio)
            .add_systems(OnEnter(AppState::InGame), setup);

        if cfg!(feature = "colliders") {
            app.add_plugins(PhysicsDebugPlugin::default());
        }
    }
}

fn stop_audio(audio: Res<Audio>) {
    audio.stop();
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
        assets_handler.add_glb(&mut game_assets.car, "models/tower_car.glb");
        assets_handler.add_animation(&mut game_assets.drive_animation,"models/tower_car.glb#Animation0");
        assets_handler.add_glb(&mut game_assets.tower_01, "models/tower.glb");
        assets_handler.add_glb(&mut game_assets.skybox, "models/skybox.glb");

        assets_handler.add_material(&mut game_assets.smoke_image, "textures/smoke.png", true);
        assets_handler.add_material(
            &mut game_assets.background_image,
            "textures/ingame_background.png", 
            false,
        );

        assets_handler.add_audio(&mut game_assets.sfx_car, "audio/car_01.ogg");
        assets_handler.add_audio(&mut game_assets.sfx_car_idle, "audio/car_idle.wav");
        assets_handler.add_audio(&mut game_assets.sfx_hit, "audio/hit.wav");
        assets_handler.add_audio(&mut game_assets.sfx_lap, "audio/lap.wav");
        assets_handler.add_audio(&mut game_assets.sfx_shot, "audio/shot.wav");
        assets_handler.add_audio(&mut game_assets.sfx_tower, "audio/tower.wav");
        assets_handler.add_audio(&mut game_assets.bgm_1, "audio/bgm.ogg");
        assets_handler.add_audio(&mut game_assets.bgm_2, "audio/end_bgm.ogg");

        assets_handler.add_standard_mesh(&mut game_assets.smoke, Mesh::from(shape::Plane { size: 0.5, subdivisions: 0 }));
        assets_handler.add_standard_mesh(&mut game_assets.hit_particle, Mesh::from(shape::Cube { size: 0.25, }));

        assets_handler.add_mesh(
            &mut game_assets.cannon.mesh,
            "models/cannon.gltf#Mesh0/Primitive0",
        );
    }
}

#[derive(Component, Clone)]
pub struct CleanupMarker;

#[derive(Component)]
pub struct Track;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<assets::GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    mut game_state: ResMut<game_settings::GameState>,
    mut shader_materials: shaders::ShaderMaterials,
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
                                Track,
                                Collider::trimesh_from_mesh(mesh).unwrap(), 
                                CollisionLayers::new([collisions::Layer::Ground], [collisions::Layer::Kart, collisions::Layer::Bullet]),
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

    if game_state.enable_background {
        if let Some(gltf) = assets_gltf.get(&game_assets.skybox) {
            let material = shader_materials
                .custom_materials
//              .add(shaders::BackgroundMaterial {
//                  texture: game_assets.background_image.image.clone(),
//                  color: Color::rgba(1., 1., 1., 1.0),
//                  x_scroll_speed: 1.0,
//                  y_scroll_speed: 1.0,
//                  scale: 1.0,
//              });
                .add(shaders::CustomMaterial {
                    color: Color::BLUE,
                    color_texture: Some(game_assets.background_image.image.clone()),
                    alpha_mode: AlphaMode::Blend,
                });

            let alpha_material = materials.add(StandardMaterial {
                base_color: Color::rgba(1., 1., 1., 0.),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            });
            commands.spawn((
                util::scene_hook::HookedSceneBundle {
                    scene: SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        ..default()
                    },
                    hook: util::scene_hook::SceneHook::new(move |cmds, hook_data| {
                        if let Some(name) = hook_data.name {
                            let name = name.as_str();

                            if name.contains("Cube") {
                                cmds.insert((
                                    material.clone(),
                                    alpha_material.clone(),
                                    bevy::pbr::NotShadowCaster,
                                    bevy::pbr::NotShadowReceiver,
                                ));
                            }
                        }
                    }),
                },
                CleanupMarker,
            ));
        }
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
                shadows_enabled: game_state.enable_shadows,
                ..Default::default()
            },
            ..Default::default()
        },
        CleanupMarker,
    ));

    commands.add(camera::SpawnCamera { cleanup_marker: CleanupMarker });

    #[cfg(feature = "debug")]
    next_ingame_state.set(IngameState::InGame);

    #[cfg(not(feature = "debug"))]
    next_ingame_state.set(IngameState::PreGame);
}
