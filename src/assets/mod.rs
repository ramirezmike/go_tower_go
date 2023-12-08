use bevy::{prelude::*, gltf::Gltf};

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
    pub sfx_1: Handle<AudioSource>,
    pub sfx_2: Handle<AudioSource>,

    pub car: Handle<Gltf>,
    pub track: Handle<Gltf>,
    pub tower_01: Handle<Gltf>,
    pub cannon: GameMesh,
    pub bevy_icon: GameTexture,


    pub title_screen_logo: GameTexture,
    pub smoke_image: GameTexture,
    pub smoke: Handle<Mesh>,

    pub drive_animation: Handle<AnimationClip>,
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
