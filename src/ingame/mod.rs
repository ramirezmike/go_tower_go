use bevy::{prelude::*, ecs::system::{Command, SystemState}, gltf::Gltf, };
use bevy_xpbd_3d::prelude::*;
use bevy_turborand::prelude::*;
use crate::{assets, AppState, util, };

mod camera; 

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin,))
            .add_systems(OnEnter(AppState::InGame), setup);

        if cfg!(feature = "lines") {
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
                            ));
                        }
                    }
                })
            }, 
        ));
    }

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }, 
    ));

    commands.add(camera::SpawnCamera { cleanup_marker: CleanupMarker });
}
