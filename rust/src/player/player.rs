use std::f64::consts::FRAC_PI_4;

use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Timer, Without};
use gdnative::api::{
    AnimationNodeStateMachinePlayback, AnimationPlayer, AnimationTree, CollisionShape2D,
    KinematicBody2D,
};
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::{GodotObjInstance, GodotObjRef, PlayingGame};
use gdrust::ecs::engine_sync::resources::PhysicsDelta;
use gdrust::unsafe_functions::{InstanceExt, NodeExt, ObjectExt, RefExt};

use crate::{
    components::{Acceleration, Friction, Roll, Stats, Velocity},
    delect_box::hit_box::HitBox,
    delect_box::hurt_box::HurtBox,
    with_world,
};

const ATTACK_ANIMATION_LEN: f32 = 0.4;
const ROLL_ANIMATION_LEN: f32 = 0.5;

/// player state.
/// This is the state of the player.
/// It is used to determine the player's state.
#[derive(Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    MOVE,
    ATTACK,
    ROLL,
}

/// Player component.
/// This is the component of the player.
#[derive(Component, NativeClass, Default, Copy, Clone)]
#[inherit(KinematicBody2D)]
#[user_data(user_data::RwLockData<Player>)]
pub struct Player {
    state: PlayerState,
}

#[methods]
impl Player {
    fn new(_owner: TRef<KinematicBody2D>) -> Self {
        Default::default()
    }

    #[export]
    fn _ready(&self, owner: TRef<KinematicBody2D>) {
        let animation_tree = owner.expect_node::<AnimationTree>("AnimationTree");
        let animation_player = owner.expect_node::<AnimationPlayer>("AnimationPlayer");

        animation_tree.set_active(true);

        owner
            .expect_node::<CollisionShape2D>("HixboxPivot/SwordHitbox/CollisionShape2D")
            .set_disabled(true);

        // Add player to ECS.
        with_world(|w| {
            let animation_state = animation_tree
                .get("parameters/playback")
                .try_to_object::<AnimationNodeStateMachinePlayback>()
                .expect("Could not get AnimationNodeStateMachinePlayback");

            w.spawn()
                .insert(GodotObjInstance::new(
                    owner.expect_as_instance::<Player>().claim(),
                ))
                .insert(GodotObjInstance::new(
                    owner
                        .expect_instance::<Acceleration>("Acceleration")
                        .claim(),
                ))
                .insert(GodotObjInstance::new(
                    owner.expect_instance::<Friction>("Friction").claim(),
                ))
                .insert(GodotObjInstance::new(
                    owner.expect_instance::<Roll>("Roll").claim(),
                ))
                .insert(GodotObjInstance::new(
                    owner.expect_instance::<Stats>("Stats").claim(),
                ))
                .insert(GodotObjInstance::new(
                    owner
                        .expect_instance::<HitBox>("HixboxPivot/SwordHitbox")
                        .claim(),
                ))
                .insert(GodotObjInstance::new(
                    owner.expect_instance::<HurtBox>("Hurtbox").claim(),
                ))
                .insert(GodotObjRef::new(animation_tree.claim()))
                .insert(GodotObjRef::new(animation_player.claim()))
                .insert(GodotObjRef::new(animation_state))
                .insert(Velocity::default())
                .insert(PlayingGame);
        });
    }
}

/// Player state system.
/// This system is used to determine the player's state.
pub fn player_state_system(player: Query<&GodotObjInstance<Player>>) {
    let input = Input::godot_singleton();

    for player in player.iter() {
        let player = player.expect_safe();
        let state = player.map(|p, _| p.state).unwrap_or(PlayerState::MOVE);

        if state == PlayerState::MOVE {
            if input.is_action_just_pressed("attack", false) {
                player
                    .map_mut(|p, _| p.state = PlayerState::ATTACK)
                    .unwrap();
            }
            if input.is_action_just_pressed("roll", false) {
                player.map_mut(|p, _| p.state = PlayerState::ROLL).unwrap();
            }
        }
    }
}

pub fn player_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut animation: Query<(Entity, &mut Velocity, &mut Timer, &GodotObjInstance<Player>)>,
) {
    for (entity, mut velocity, mut timer, player) in animation.iter_mut() {
        let player = player.expect_safe();

        timer.tick(time.delta());
        if timer.finished() {
            let state = player
                .map(|p, _| p.state.clone())
                .unwrap_or(PlayerState::MOVE);
            if state == PlayerState::ROLL {
                velocity.velocity = Vector2::ZERO;
            }
            commands.entity(entity).remove::<Timer>();
            player.map_mut(|p, _| p.state = PlayerState::MOVE).unwrap();
        }
    }
}

/// Player Move System.
/// This system is used to change player's velocity.
pub fn player_move_system(
    delta: Res<PhysicsDelta>,
    mut player: Query<(
        &mut Velocity,
        &GodotObjInstance<HitBox>,
        &GodotObjRef<AnimationTree>,
        &GodotObjRef<AnimationNodeStateMachinePlayback>,
        &GodotObjInstance<Acceleration>,
        &GodotObjInstance<Friction>,
        &GodotObjInstance<Roll>,
        &GodotObjInstance<Player>,
    )>,
) {
    for (
        mut velocity,
        hitbox,
        animation_tree,
        animation_state,
        acceleration,
        friction,
        roll,
        player,
    ) in player.iter_mut()
    {
        let state = player.expect_safe().map(|p, _| p.state.clone()).unwrap();
        if state == PlayerState::MOVE {
            let hitbox = hitbox.expect_safe();
            let roll = roll.expect_safe();
            let animation_tree = animation_tree.expect_safe();
            let animation_state = animation_state.expect_safe();
            let acceleration = acceleration.expect_safe();

            let input = Input::godot_singleton();
            let mut input_vector = Vector2::new(
                input.get_action_strength("ui_right", false) as f32
                    - input.get_action_strength("ui_left", false) as f32,
                input.get_action_strength("ui_down", false) as f32
                    - input.get_action_strength("ui_up", false) as f32,
            );

            if input_vector != Vector2::ZERO {
                input_vector = input_vector.normalized();
                hitbox
                    .map_mut(|h, _| h.knockback_vector = input_vector)
                    .unwrap();
                roll.map_mut(|r, _| r.roll_velocity = input_vector).unwrap();

                animation_tree.set("parameters/Idle/blend_position", input_vector);
                animation_tree.set("parameters/Run/blend_position", input_vector);
                animation_tree.set("parameters/Attack/blend_position", input_vector);
                animation_tree.set("parameters/Roll/blend_position", input_vector);

                animation_state.travel("Run");

                velocity.velocity = velocity.move_toward(
                    acceleration.map(|a, _| input_vector * a.max_speed).unwrap(),
                    acceleration
                        .map(|a, _| a.acceleration_speed * delta.value)
                        .unwrap(),
                );
            } else {
                animation_state.travel("Idle");

                let friction = friction.expect_safe();
                velocity.velocity = velocity.move_toward(
                    Vector2::ZERO,
                    friction.map(|f, _| f.friction * delta.value).unwrap(),
                );
            }
        }
    }
}

/// Player Attack System.
/// This system is used to attack the player.
pub fn player_attack_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &GodotObjRef<AnimationNodeStateMachinePlayback>,
            &GodotObjInstance<Player>,
        ),
        Without<Timer>,
    >,
) {
    for (entity, mut velocity, animation_state, player) in query.iter_mut() {
        let state = player.expect_safe().map(|p, _| p.state.clone()).unwrap();
        if state == PlayerState::ATTACK {
            velocity.velocity = Vector2::ZERO;
            animation_state.expect_safe().travel("Attack");

            commands
                .entity(entity)
                .insert(Timer::from_seconds(ATTACK_ANIMATION_LEN, false));
        }
    }
}

/// Player Roll System.
/// This system is used to roll the player.
pub fn player_roll_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &GodotObjRef<AnimationNodeStateMachinePlayback>,
            &GodotObjInstance<Roll>,
            &GodotObjInstance<Player>,
        ),
        Without<Timer>,
    >,
) {
    for (entity, mut velocity, animation_state, roll, player) in query.iter_mut() {
        let state = player.expect_safe().map(|p, _| p.state.clone()).unwrap();
        if state == PlayerState::ROLL {
            let roll = roll.expect_safe();

            velocity.velocity = roll.map(|r, _| r.roll_velocity * r.roll_speed).unwrap();
            animation_state.expect_safe().travel("Roll");

            commands
                .entity(entity)
                .insert(Timer::from_seconds(ROLL_ANIMATION_LEN, false));
        }
    }
}

/// Player Move System.
/// This system is used to move the player.
pub fn player_movement_system(mut query: Query<(&mut Velocity, &GodotObjInstance<Player>)>) {
    for (mut velocity, player) in query.iter_mut() {
        velocity.velocity = player
            .expect_safe()
            .map(|_, o| {
                o.move_and_slide(velocity.velocity, Vector2::ZERO, false, 4, FRAC_PI_4, true)
            })
            .unwrap();
    }
}
