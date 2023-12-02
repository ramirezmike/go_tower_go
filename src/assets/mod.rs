use bevy::{prelude::*, gltf::Gltf};

pub mod loader;
mod loader_ext;

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

    pub bgm_1: Handle<AudioSource>,
    pub sfx_1: Handle<AudioSource>,

    pub car: Handle<Gltf>,
    pub track: Handle<Gltf>,
    pub bevy_icon: GameTexture,

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
