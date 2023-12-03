use bevy::prelude::*;
use crate::{AppState, };
use super::controller;

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bots.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Bot;

fn move_bots(
    bots: Query<(Entity, &Bot)>,
    mut movement_event_writer: EventWriter<controller::MovementEvent>,
) {
    for (entity, _bot) in &bots {
        println!("sending move");
        movement_event_writer.send(controller::MovementEvent {
            entity,
            action: controller::MovementAction::Gas,
        });
    }
}
