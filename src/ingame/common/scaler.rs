use bevy::{prelude::*, ecs::system::{Command,SystemState}};

pub struct ScalerPlugin;
impl Plugin for ScalerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_scalers);
    }
}


#[derive(Component, Default)]
pub struct Scaler {
    size: Vec3,
    current_time: f32,
    scale_up_time: f32,
    scale_down_time: f32,
    has_peaked: bool,
    has_started: bool,
}

impl Scaler {
    pub fn new(size: Vec3, scale_up_time: f32, scale_down_time: f32, has_started: bool) -> Self {
        Scaler {
            size,
            current_time: 0.,
            scale_up_time,
            scale_down_time,
            has_peaked: false,
            has_started,
        }
    }
}

fn handle_scalers(
    mut commands: Commands,
    mut scalers: Query<(Entity, &mut Transform, &mut Scaler)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut scaler) in &mut scalers {
        let initial = Vec3::splat(1.);
        if !scaler.has_started {
            transform.scale = initial;
            scaler.has_started = true;
        }
        scaler.current_time += time.delta_seconds();
        if !scaler.has_peaked {
            transform.scale = transform.scale.lerp(scaler.size, scaler.current_time / scaler.scale_up_time);

            if scaler.current_time >= scaler.scale_up_time {
                scaler.has_peaked = true;
                transform.scale = scaler.size;
                scaler.current_time = 0.;
            }
        } else {
            transform.scale = transform.scale.lerp(initial, scaler.current_time / scaler.scale_down_time);

            if scaler.current_time >= scaler.scale_down_time {
                transform.scale = initial;
                commands.entity(entity).remove::<Scaler>();
            }
        }
    }
}
