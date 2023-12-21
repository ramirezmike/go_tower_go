use bevy::{prelude::*, gltf::Gltf};
use std::collections::HashMap;

pub mod loader;
mod loader_ext;
use bevy_kira_audio::AudioSource;

pub use self::loader_ext::*;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default())
            .add_plugins(loader::AssetLoadingPlugin);
    }
}

#[derive(Component)]
pub struct AnimationLink {
    pub entity: Entity,
}

#[derive(Default, Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,

    pub title_bgm: Handle<AudioSource>,
    pub bgm_1: Handle<AudioSource>,
    pub bgm_2: Handle<AudioSource>,
    pub sfx_1: Handle<AudioSource>,
    pub sfx_2: Handle<AudioSource>,

    pub sfx_car: Handle<AudioSource>,
    pub sfx_car_idle: Handle<AudioSource>,
    pub sfx_hit: Handle<AudioSource>,
    pub sfx_lap: Handle<AudioSource>,
    pub sfx_shot: Handle<AudioSource>,
    pub sfx_tower: Handle<AudioSource>,

    pub car: Handle<Gltf>,
    pub track: Handle<Gltf>,
    pub tower_01: Handle<Gltf>,
    pub cannon: GameMesh,
    pub bevy_icon: GameTexture,
    pub skybox: Handle<Gltf>,
    pub background_image: GameTexture,
    pub kart_colors: HashMap<usize, Handle<StandardMaterial>>,

    pub title_screen_logo: GameTexture,
    pub smoke_image: GameTexture,
    pub smoke: Handle<Mesh>,
    pub hit_particle: Handle<Mesh>,
    pub bullet_mesh: Handle<Mesh>,

    pub drive_animation: Handle<AnimationClip>,

    pub controls_gamepad: GameTexture,
    pub controls_keyboard: GameTexture,
    pub instructions: GameTexture,
}

impl GameAssets {
    pub fn add_kart_color(&mut self, material: Handle<StandardMaterial>) -> usize {
        let length = self.kart_colors.len();
        self.kart_colors.insert(length, material);

        length
    }
}

#[derive(Default)]
pub struct GameMesh {
    pub mesh: Handle<Mesh>,
    pub texture: GameTexture,
}

#[derive(Default)]
pub struct GameTexture {
    pub material: Handle<StandardMaterial>,
    pub image: Handle<Image>,
}
