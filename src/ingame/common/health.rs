use bevy::{prelude::*, ecs::system::{Command,SystemState}};
use crate::{assets, ingame, AppState};
use bevy_xpbd_3d::PhysicsSet;
use bevy::transform::TransformSystem;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (scale_healthbars, handle_hit_events).run_if(in_state(AppState::InGame)))
            .add_systems(
                PostUpdate,
                (healthbar_follow_parent, handle_healthbar_view)
                    .chain()
                    .run_if(in_state(AppState::InGame))
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_event::<HealthHitEvent>();
    }
}

const HEALTHBAR_SIZE: f32 = 2.0;

#[derive(Event)]
pub struct HealthHitEvent {
    pub entity: Entity,
    pub hit_points: usize
}

fn handle_hit_events(
    mut health_hit_event_reader: EventReader<HealthHitEvent>,
    mut healths: Query<&mut Health>,
) {
    for event in health_hit_event_reader.read() {
        if let Ok(mut health) = healths.get_mut(event.entity) {
            health.subtract(event.hit_points);
        }
    }
}

#[derive(Component)]
pub struct Health {
    health_points: usize,
    max_health: usize,
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.health_points == 0
    }

    pub fn is_alive(&self) -> bool {
        self.health_points > 0
    }

    pub fn subtract(&mut self, hp: usize) {
        self.health_points = self.health_points.saturating_sub(hp);
    }

    pub fn add(&mut self, hp: usize) {
        self.health_points = self.health_points.saturating_add(hp).min(self.max_health);
    }
}

#[derive(Component)]
pub struct HealthBar {
    pub parent: Entity,
    pub offset: Vec3,
}

fn scale_healthbars(
    mut healthbars: Query<(&mut Transform, &HealthBar)>,
    parents: Query<&Health>,
) {
    for (mut healthbar_transform, healthbar) in healthbars.iter_mut() {
        if let Ok(parent) = parents.get(healthbar.parent) {
            healthbar_transform.scale.x = (parent.health_points as f32 / parent.max_health as f32) * HEALTHBAR_SIZE;
        }
    }
}

fn healthbar_follow_parent(
    mut healthbars: Query<(&mut Transform, &HealthBar)>,
    parents: Query<&Transform, Without<HealthBar>>,
) {
    for (mut healthbar_transform, healthbar) in healthbars.iter_mut() {
        if let Ok(parent) = parents.get(healthbar.parent) {
            healthbar_transform.translation = parent.translation + healthbar.offset;
        }
    }
}
fn handle_healthbar_view(
    mut healthbars: Query<&mut Transform, With<HealthBar>>,
    camera: Query<&Transform, (With<Camera>, Without<HealthBar>)>,
) {
    if let Ok(camera) = camera.get_single() {
        for mut healthbar in healthbars.iter_mut() {
            healthbar.look_at(camera.translation, Vec3::Y);
        }
    }
}

pub struct HealthBarSpawner<C: Component + Clone> {
    pub health_points: usize,
    pub parent: Entity,
    pub cleanup_marker: C,
    pub offset: Vec3,
}

impl<C: Component + Clone> Command for HealthBarSpawner<C> {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            assets::loader::AssetsHandler,
            Res<assets::GameAssets>,
            Query<(&Transform, )>,
        )> = SystemState::new(world);

        let (mut assets_handler, game_assets, transforms) = system_state.get_mut(world);

        let green_bar_material = assets_handler.materials.add(Color::GREEN.into()).clone();
        let mesh = game_assets.smoke.clone_weak(); 

        if let Ok((transform, )) = transforms.get(self.parent) {
            let mut transform = transform.clone();
            transform.translation += self.offset;
            let mut healthbar= world.spawn( ( SpatialBundle::from_transform(transform), 
                    HealthBar {
                        parent: self.parent,
                        offset: self.offset,
                    },
            ),);
            let healthbar_id = healthbar.id();
            healthbar.with_children(|parent| {
                    parent.spawn((PbrBundle {
                        mesh ,
                        material: green_bar_material.clone(),
                        transform: Transform::from_rotation(
                            Quat::from_axis_angle(Vec3::X, (3.0 * std::f32::consts::PI) / 2.0))
                            .with_scale(Vec3::new(HEALTHBAR_SIZE, 1.0, 0.3)),
                        ..Default::default()
                    },
                    bevy::pbr::NotShadowCaster,
                    ingame::CleanupMarker,
                ));
            });

            if let Some(mut parent) = world.get_entity_mut(self.parent) {
                parent.insert(Health {
                    health_points: self.health_points,
                    max_health: self.health_points,
                });
            }
        }
    }
}
