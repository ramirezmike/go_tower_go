use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use bevy::render::primitives::Aabb;
use bevy_turborand::prelude::*;
use std::f32::consts::TAU;
use bevy_xpbd_3d::{math::*, prelude::*};
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use crate::{assets, util, AppState, };
use super::{bot, controller, player, config, race, points};

#[derive(Component)]
pub struct Kart;

pub struct KartSpawner<C: Component + Clone> {
    pub global_transform: GlobalTransform,
    pub aabb: Aabb,
    pub cleanup_marker: C
}
impl<C: Component + Clone> Command for KartSpawner<C> {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            Res<assets::GameAssets>,
            Res<Assets<Gltf>>,
            ResMut<GlobalRng>,
            Query<Entity, With<player::Player>>,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, assets_gltf, mut global_rng, players) = system_state.get_mut(world);
        let matrix = self.global_transform.compute_matrix();
        let spawn_point = matrix.transform_point3(self.aabb.center.into());
        let rand = global_rng.f32_normalized();

        let count_of_spawned_players = players.iter().count();

        let gltf = assets_gltf.get(&game_assets.car);
        if let Some(gltf) = gltf {
            let scene = gltf.scenes[0].clone();
            let mut entity = world.spawn((
                Kart,
                util::scene_hook::HookedSceneBundle {
                    scene: SceneBundle {
                        scene,
                        transform: Transform::from_translation(spawn_point + Vec3::new(0., 0.5, 0.)).with_rotation(Quat::from_axis_angle(Vec3::Y, TAU * 0.75)),
                        ..default()
                    },
                    hook: util::scene_hook::SceneHook::new(move |cmds, hook_data| {
                        if let Some(mesh) = hook_data.mesh {
                            cmds.insert(
                            OutlineBundle {
                                outline: OutlineVolume {
                                    visible: true,
                                    width: 1.0,
                                    colour: Color::BLACK,
                                },
                                mode: OutlineMode::RealVertex,
                                ..default()
                            });
                        }
                    })
                },
                race::NextWayPoint(race::WayPoints::Quarter),
                race::LapCounter(1),
                points::Points(8),
                self.cleanup_marker,
                controller::CommonControllerBundle::new(Collider::capsule(0.3, 0.6), Vector::NEG_Y * 9.81 * 1.5)
            ));

            if count_of_spawned_players >= config::NUMBER_OF_PLAYERS {
                entity.insert(bot::Bot::new(rand));
            } else {
                entity.insert((player::Player, controller::CharacterControllerKeyboard,));
            }
        }
    }
}
