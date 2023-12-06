use bevy::prelude::*;
use bevy::ecs::system::{Command, SystemState};
use bevy_camera_shake::Shake3d;

const DEFAULT_TRAUMA: f32 = 0.25;
pub struct CameraShake {
    trauma: f32
}

impl CameraShake {
    pub fn new(trauma: f32) -> Self {
        CameraShake {
            trauma
        }
    }
}

impl Default for CameraShake {
    fn default() -> Self {
        CameraShake::new(DEFAULT_TRAUMA)
    }
}

impl Command for CameraShake {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<Query<&mut Shake3d>> = SystemState::new(world);

        for mut shakeable in &mut system_state.get_mut(world) {
            shakeable.trauma = f32::min(shakeable.trauma + self.trauma, 1.0);
        }
    }
}
