use bevy::{
    ecs::system::EntityCommands, prelude::*, render::primitives::Aabb,
    scene::SceneInstance,
};
use std::marker::PhantomData;

pub struct HookPlugin;
impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_hooks);
    }
}

#[derive(Component, Debug)]
pub struct SceneHooked;

#[derive(Bundle)]
pub struct HookedSceneBundle {
    pub hook: SceneHook,
    pub scene: SceneBundle,
}

pub struct HookData<'a, 'w> {
    pub mesh: Option<&'a Mesh>,
    pub global_transform: Option<&'a GlobalTransform>,
    pub transform: Option<&'a Transform>,
    pub material: Option<&'a StandardMaterial>,
    pub aabb: Option<&'a Aabb>,
    pub name: Option<&'a Name>,

    phantom: PhantomData<&'w ()>,
}

#[derive(Component)]
pub struct SceneHook {
    hook: Box<dyn Fn(&mut EntityCommands, HookData) + Send + Sync + 'static>,
}
impl SceneHook {
    pub fn new<F: Fn(&mut EntityCommands, HookData) + Send + Sync + 'static>(hook: F) -> Self {
        Self {
            hook: Box::new(hook),
        }
    }
}

#[derive(Component)]
pub struct SceneOnComplete {
    on_complete: Box<
        dyn Fn(&mut Commands,) + Send + Sync + 'static,
    >,
}
impl SceneOnComplete {
    pub fn new<
        F: Fn(
                &mut Commands,
            ) + Send
            + Sync
            + 'static,
    >(
        f: F,
    ) -> Self {
        Self {
            on_complete: Box::new(f),
        }
    }
}

pub fn run_hooks(
    unloaded_instances: Query<
        (Entity, &SceneInstance, &SceneHook, Option<&SceneOnComplete>),
        Without<SceneHooked>,
    >,
    scene_manager: Res<SceneSpawner>,
    meshes: Res<Assets<Mesh>>,
    standard_materials: Res<Assets<StandardMaterial>>,
    components: Query<(
        Option<&GlobalTransform>,
        Option<&Transform>,
        Option<&Aabb>,
        Option<&Handle<Mesh>>,
        Option<&Handle<StandardMaterial>>,
        Option<&Name>,
    )>,
    mut cmds: Commands,
) {
    for (entity, instance, hooked, maybe_on_complete) in &unloaded_instances {
        for entity in scene_manager.iter_instance_entities(**instance) {
            if let Ok((global_transform, transform, aabb, mesh_handle, material_handle, name)) = components.get(entity) {
                let mesh = mesh_handle.and_then(|m| meshes.get(m));
                let material = material_handle.and_then(|m| standard_materials.get(m));

                let hook_data = HookData {
                    mesh,
                    material,
                    global_transform,
                    transform,
                    aabb,
                    name,
                    phantom: PhantomData::<&()>,
                };

                let mut cmd = cmds.entity(entity);
                (hooked.hook)(&mut cmd, hook_data);
            }
        }

        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).insert(SceneHooked);
            if let Some(on_complete) = maybe_on_complete {
                (on_complete.on_complete)(&mut cmds);
            }
        }
    }
}
