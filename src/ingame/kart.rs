use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use bevy::render::primitives::Aabb;
use bevy_turborand::prelude::*;
use std::f32::consts::TAU;
use bevy_xpbd_3d::{math::*, prelude::*};
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use crate::{assets, util, AppState, };
use super::{bot, controller, player, config};

#[derive(Component)]
pub struct Kart;

pub struct KartSpawner {
    pub global_transform: GlobalTransform,
    pub aabb: Aabb,
}
impl Command for KartSpawner {
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

        let mesh = assets_handler.meshes.add(Mesh::from(shape::Cube { size: 1.0 })); 
        let material = assets_handler.materials.add(Color::rgb_u8(255, 144, 124).into());
        let count_of_spawned_players = players.iter().count();

        let mut entity = world.spawn((
            PbrBundle {
                mesh, 
                material,
                transform: Transform::from_translation(spawn_point + Vec3::new(0., 0.5, 0.)).with_rotation(Quat::from_axis_angle(Vec3::Y, TAU * 0.75)),
                ..default()
            },
            Kart,
            OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    width: 1.0,
                    colour: Color::BLACK,
                },
                mode: OutlineMode::RealVertex,
                ..default()
            },
            controller::CommonControllerBundle::new(Collider::capsule(0.3, 0.4), Vector::NEG_Y * 9.81 * 2.0)
        ));

        if count_of_spawned_players >= config::NUMBER_OF_PLAYERS {
            entity.insert(bot::Bot::new(rand));
        } else {
            entity.insert((player::Player, controller::CharacterControllerKeyboard,));
        }
    }
}
