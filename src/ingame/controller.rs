// Adapted from bevy_xpbd_3d 🙏🙏🙏🙏🙏  
use bevy::{ecs::query::Has, prelude::*};
use bevy_xpbd_3d::{math::*, prelude::*, SubstepSchedule, SubstepSet};
use crate::{ingame::config, ingame::tower, AppState, IngameState, ingame::path, ingame::player};
use bevy::input::gamepad::GamepadButtonType;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementEvent>()
            .add_systems(
                Update,
                (
                    (keyboard_input,
                    gamepad_input,).run_if(in_state(AppState::InGame).and_then(in_state(IngameState::InGame))),
                    update_grounded,
                    handle_fallen,
                    apply_deferred,
                    apply_gravity,
                    movement,
                    apply_movement_damping,
                )
                    .chain(),
            )
            .add_systems(
                // Run collision handling in substep schedule
                SubstepSchedule,
                kinematic_controller_collisions.in_set(SubstepSet::SolveUserConstraints),
            );
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub struct MovementEvent {
    pub entity: Entity,
    pub action: MovementAction,
}

pub enum MovementAction {
    Gas,
    Brake,
    Turn(f32),
}

#[derive(Component)]
pub struct CommonController;

/// A marker component indicating that an entity is using the keyboard
#[derive(Component)]
pub struct CharacterControllerKeyboard;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component, Debug)]
#[component(storage = "SparseSet")]
pub struct LastGrounded {
    translation: Vec3,
    delta: f32,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Braking;
/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(pub Scalar);

/// The deceleration used for character movement.
#[derive(Component)]
pub struct MovementDeceleration(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The damping factor used for slowing down rotation.
#[derive(Component)]
pub struct RotationDampingFactor(Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
pub struct ControllerGravity(Vector);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CommonControllerBundle {
    common_controller: CommonController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    deceleration: MovementDeceleration,
    rotation_damping: RotationDampingFactor,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        deceleration: Scalar,
        damping: Scalar,
        rotation_damping: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            deceleration: MovementDeceleration(deceleration),
            damping: MovementDampingFactor(damping),
            rotation_damping: RotationDampingFactor(rotation_damping),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(config::BASE_ACCELERATION, config::BASE_DECELERATION, config::MOVEMENT_DAMPING, config::ROTATION_DAMPING,  PI * config::MAX_SLOPE)
    }
}

impl CommonControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            common_controller: CommonController,
            rigid_body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Vector::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            gravity: ControllerGravity(gravity),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        deceleration: Scalar,
        damping: Scalar,
        rotation_damping: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, deceleration, damping, rotation_damping, max_slope_angle);
        self
    }
}

/// Sends [`MovementEvent`] events based on keyboard input.
fn keyboard_input(
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    keyboard_player: Query<Entity, With<CharacterControllerKeyboard>>,
) {
    for entity in &keyboard_player {
        let up = keyboard_input.any_pressed([KeyCode::K, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::J, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, ]);
        let right = keyboard_input.any_pressed([KeyCode::D, ]);

        let right_trigger = keyboard_input.just_pressed(KeyCode::L) || keyboard_input.just_pressed(KeyCode::Right);
        let left_trigger = keyboard_input.just_pressed(KeyCode::H) || keyboard_input.just_pressed(KeyCode::Left);

        if right_trigger {
            commands.add(tower::TowerSpawner { entity });
        }

        if up {
            movement_event_writer.send(MovementEvent {
                entity,
                action: MovementAction::Gas,
            });
        }
        if down {
            movement_event_writer.send(MovementEvent {
                entity,
                action: MovementAction::Brake,
            });
        }
        if left {
            movement_event_writer.send(MovementEvent {
                entity,
                action: MovementAction::Turn(1.),
            });
        } else if right {
            movement_event_writer.send(MovementEvent {
                entity,
                action: MovementAction::Turn(-1.),
            });
        }
    }
}

/// Sends [`MovementEvent`] events based on gamepad input.
fn gamepad_input(
    mut commands: Commands,
    mut movement_event_writer: EventWriter<MovementEvent>,
    keyboard_player: Query<Entity, With<CharacterControllerKeyboard>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
) {
    // TODO: refactor this for multiplayer?
    for entity in &keyboard_player {
        for gamepad in gamepads.iter() {
            let axis_lx = GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            };
            let axis_ly = GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickY,
            };


            if buttons.just_pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::West }) ||
                buttons.just_pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::North }) || 
                buttons.just_pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::LeftTrigger }) {
                commands.add(tower::TowerSpawner { entity });
            }

            if buttons.pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::South }) {
                movement_event_writer.send(MovementEvent {
                    entity,
                    action: MovementAction::Gas,
                });
            }

            if buttons.pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::East }) 
            || buttons.pressed(GamepadButton { gamepad,  button_type: GamepadButtonType::RightTrigger }) {
                movement_event_writer.send(MovementEvent {
                    entity,
                    action: MovementAction::Brake,
                });
            }
            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                movement_event_writer.send(MovementEvent {
                    entity,
                    action: MovementAction::Turn(-x),
                });
            }
        }
    }
}

fn handle_fallen(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut LastGrounded), With<player::Player>>,
    path_manager: Res<path::PathManager>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut last_grounded) in &mut query {
        last_grounded.delta += time.delta_seconds();
        if last_grounded.delta > 2. {
            transform.translation = path_manager.get_closest_index(last_grounded.translation)
                                        .and_then(|i| path_manager.get_next(i))
                                        .map(|i| path_manager.get(i))
                                        .map(|v| v + (Vec3::Y * 3.))
                                        .unwrap_or(last_grounded.translation);
            commands.entity(entity).remove::<LastGrounded>();
        }
    }

}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Transform, &Rotation, Option<&MaxSlopeAngle>, Has<LastGrounded>),
        With<CommonController>,
    >,
) {
    for (entity, hits, transform, rotation, max_slope_angle, has_last_grounded) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
            commands.entity(entity).remove::<LastGrounded>();
        } else {
            commands.entity(entity).remove::<Grounded>();
            if !has_last_grounded {
                commands.entity(entity).insert(LastGrounded {
                    translation: transform.translation,
                    delta: 0.
                });
            }
        }
    }
}

/// Responds to [`MovementEvent`] events and moves character controllers accordingly.
fn movement(
    mut commands: Commands,
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementEvent>,
    mut controllers: Query<(Entity,
        &MovementAcceleration,
        &MovementDeceleration,
        &RotationDampingFactor,
        &mut LinearVelocity,
        &mut Transform,
        Has<Grounded>,
    )>,
) {
    let delta_time = time.delta_seconds();

    for event in movement_event_reader.read() {
        if let Ok((entity, movement_acceleration, movement_deceleration, rotation_damping, mut linear_velocity, mut transform, is_grounded)) = controllers.get_mut(event.entity) {
            match event.action {
                MovementAction::Gas => {
                    commands.entity(entity).remove::<Braking>();
                    let direction = transform.forward(); 
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z += direction.z * movement_acceleration.0 * delta_time;
                },
                MovementAction::Brake => {
                    commands.entity(entity).insert(Braking);
                    let direction = transform.forward(); 
                    linear_velocity.x -= direction.x * (movement_acceleration.0 * movement_deceleration.0) * delta_time;
                    linear_velocity.z -= direction.z * (movement_acceleration.0 * movement_deceleration.0) * delta_time;
                },
                MovementAction::Turn(direction) => {
                    transform.rotate_local_y(direction * (linear_velocity.length() * rotation_damping.0) * delta_time);
                }
            }
        }
    }
}

/// Applies [`ControllerGravity`] to character controllers.
fn apply_gravity(
    time: Res<Time>,
    mut controllers: Query<(&ControllerGravity, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds();

    for (gravity, mut linear_velocity) in &mut controllers {
        linear_velocity.0 += gravity.0 * delta_time;
    }
}

/// Slows down movement in the XZ plane.
pub fn apply_movement_damping(time: Res<Time>, mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    let delta_time = time.delta_seconds();
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
//      let friction:f32 = 0.05;
//      linear_velocity.0 *= friction.powf(time.delta_seconds());

        linear_velocity.x *= 1. - delta_time;
        linear_velocity.z *= 1. - delta_time;
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system performs very basic collision response for kinematic
/// character controllers by pushing them along their contact normals
/// by the current penetration depths.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (
            &RigidBody,
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
            Option<&MaxSlopeAngle>,
        ),
        With<CommonController>,
    >,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // If the collision didn't happen during this substep, skip the collision
        if !contacts.during_current_substep {
            continue;
        }

        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;
        let (rb, mut position, rotation, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(collider_parent1.get()) {
                is_first = true;
                character
            } else if let Ok(character) = character_controllers.get_mut(collider_parent2.get()) {
                is_first = false;
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers
        if !rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            // Solve each penetrating contact in the manifold
            for contact in manifold.contacts.iter().filter(|c| c.penetration > 0.0) {
                position.0 += normal * contact.penetration;
            }

            // If the slope isn't too steep to walk on but the character
            // is falling, reset vertical velocity.
            if max_slope_angle.is_some_and(|angle| normal.angle_between(Vector::Y).abs() <= angle.0)
                && linear_velocity.y < 0.0
            {
                linear_velocity.y = linear_velocity.y.max(0.0);
            }
        }
    }
}

