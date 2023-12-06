use bevy::prelude::*;

pub mod health;
pub mod scaler;

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((health::HealthPlugin, scaler::ScalerPlugin));
    }
}
