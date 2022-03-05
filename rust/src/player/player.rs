use bevy::prelude::{Bundle, Commands, Component, EventReader, Query, Res, ResMut, State, With};
use gdnative::api::KinematicBody2D;
use gdnative::prelude::{Input, Ref, Vector2};
use gdrust::ecs::engine_sync::components::PlayingGame;
use gdrust::ecs::engine_sync::events::SpawnNode;
use gdrust::ecs::engine_sync::resources::PhysicsDelta;
use gdrust::gdrust_macros::{gdbundle, gdcomponent};
use gdrust::unsafe_functions::{NodeExt, RefExt};
use std::f64::consts::FRAC_PI_4;

use crate::delect_box::hit_box::HitBoxPosition;
use crate::{
    components::{Acceleration, Animation, Friction, Roll, Stats, Velocity},
    delect_box::hurt_box::HurtBox,
};

pub struct PlayerAttackAnimationFinished;
pub struct PlayerRollAnimationFinished;

/// player state.
/// This is the state of the player.
/// It is used to determine the player's state.
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub enum PlayerState {
    MOVE,
    ATTACK,
    ROLL,
}

/// Player component.
/// This is the component of the player.
#[gdcomponent(extends = KinematicBody2D)]
pub struct Player {
    #[node]
    node: Ref<KinematicBody2D>,
}

/// Player bundle.
/// This is the bundle of the player.
/// It is used to create the player.
/// It is used to add the player to the world.
#[gdbundle]
pub struct PlayerBundle {
    #[value(Player::new(node.claim()))]
    player: Player,
    #[component("Acceleration")]
    acceleration: Acceleration,
    #[component("Friction")]
    friction: Friction,
    #[component("Roll")]
    roll: Roll,
    #[value(Velocity::new(Vector2::ZERO))]
    velocity: Velocity,
    #[component("Stats")]
    stats: Stats,
    #[value(Animation::new(node))]
    animation_tree: Animation,
    #[component("HixboxPivot")]
    sword_hitbox: HitBoxPosition,
    #[component("Hurtbox")]
    hurt_box: HurtBox,
    #[value(PlayingGame)]
    in_game: PlayingGame,
}

/// Add Player Node Event.
/// This event is used to add the player node to the scene.
pub fn add_player_system(mut commands: Commands, mut event: EventReader<SpawnNode>) {
    for SpawnNode { node, name } in event.iter() {
        if name != "Player" {
            continue;
        }

        commands
            .spawn()
            .insert_bundle(PlayerBundle::new(node.clone()));
    }
}

/// Player stage system.
/// This system is used to determine the player's stage.
pub fn player_state_system(
    mut state: ResMut<State<PlayerState>>,
    mut attack_event: EventReader<PlayerAttackAnimationFinished>,
    mut roll_event: EventReader<PlayerRollAnimationFinished>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let input = Input::godot_singleton();
    if input.is_action_just_pressed("attack", false)
        && state.current().clone() != PlayerState::ATTACK
    {
        state.set(PlayerState::ATTACK).unwrap();
    }
    if input.is_action_just_pressed("roll", false) && state.current().clone() != PlayerState::ROLL {
        state.set(PlayerState::ROLL).unwrap();
    }

    for _ in attack_event.iter() {
        if state.current().clone() != PlayerState::MOVE {
            state.set(PlayerState::MOVE).unwrap();
        }
    }

    for _ in roll_event.iter() {
        if state.current().clone() != PlayerState::MOVE {
            state.set(PlayerState::MOVE).unwrap();
            (*query.single_mut()).value = Vector2::ZERO;
        }
    }
}

/// Player Move System.
/// This system is used to move the player.
pub fn player_move_system(
    delta: Res<PhysicsDelta>,
    mut query0: Query<(
        &Player,
        &mut HitBoxPosition,
        &Animation,
        &mut Velocity,
        &Acceleration,
        &Friction,
        &mut Roll,
    )>,
) {
    for (player, mut sword_hitbox, animation, mut velocity, acceleration, friction, mut roll) in
        query0.iter_mut()
    {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::new(
            input.get_action_strength("ui_right", false) as f32
                - input.get_action_strength("ui_left", false) as f32,
            input.get_action_strength("ui_down", false) as f32
                - input.get_action_strength("ui_up", false) as f32,
        );

        if input_vector != Vector2::ZERO {
            input_vector = input_vector.normalized();
            sword_hitbox.knockback_vector = input_vector;
            roll.roll_velocity = input_vector;

            let animation_tree = animation.animation_tree.expect_safe();
            let animation_state = animation.animation_state.expect_safe();

            animation_tree.set("parameters/Idle/blend_position", input_vector);
            animation_tree.set("parameters/Run/blend_position", input_vector);
            animation_tree.set("parameters/Attack/blend_position", input_vector);
            animation_tree.set("parameters/Roll/blend_position", input_vector);

            animation_state.travel("Run");

            (*velocity).value = velocity.move_toward(
                input_vector * acceleration.max_speed,
                acceleration.acceleration * delta.value,
            );
        } else {
            let animation_state = animation.animation_state.expect_safe();
            animation_state.travel("Idle");

            (*velocity).value = velocity.move_toward(Vector2::ZERO, friction.value * delta.value);
        }

        (*velocity).value = player.node.expect_safe().move_and_slide(
            (*velocity).value,
            Vector2::ZERO,
            false,
            4,
            FRAC_PI_4,
            true,
        );
    }
}

/// Player Attack System.
/// This system is used to attack the player.
pub fn player_attack_system(mut query: Query<(&mut Velocity, &Animation), With<Player>>) {
    for (mut velocity, animation) in query.iter_mut() {
        (*velocity).value = Vector2::ZERO;
        animation.animation_state.expect_safe().travel("Attack");
    }
}

/// Player Roll System.
/// This system is used to roll the player.
pub fn player_roll_system(mut query: Query<(&mut Velocity, &Animation, &Roll, &Player)>) {
    for (mut velocity, animation, roll, player) in query.iter_mut() {
        (*velocity).value = roll.roll_velocity * roll.roll_speed;
        animation.animation_state.expect_safe().travel("Roll");

        (*velocity).value = player.node.expect_safe().move_and_slide(
            (*velocity).value,
            Vector2::ZERO,
            false,
            4,
            FRAC_PI_4,
            true,
        );
    }
}
