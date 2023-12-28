use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct AimingTower;
