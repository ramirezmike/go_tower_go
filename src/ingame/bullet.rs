use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use std::f32::consts::TAU;
use bevy_xpbd_3d::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMode};
use crate::{assets, util, AppState};

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_collisions, update_bullets, ).chain().run_if(in_state(AppState::InGame)));
    }
}

pub struct BulletSpawner<C: Component + Clone>  {
    pub spawn_point: Vec3,
    pub direction: Vec3,
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
        let material = assets_handler.materials.add(Color::rgb_u8(124, 144, 255).into()).clone(); 

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
            Collider::cuboid(1.0, 1.0, 1.0),
            Bullet {
                direction: self.direction,
                speed: self.speed,
            },
            self.cleanup_marker,
        ));
    }
}

#[derive(Component)]
pub struct Bullet {
    direction: Vec3,
    speed: f32,
}

fn update_bullets(
    mut bullets: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}

fn handle_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    bullets: Query<(Entity, &Bullet)>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        match (bullets.get(contacts.entity1), bullets.get(contacts.entity2)) {
            (Ok(e), _) | (_, Ok(e)) => {
                commands.entity(e.0).despawn_recursive();
            },
            _ => ()
        }
    }
}
