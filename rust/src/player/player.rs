use std::f64::consts::FRAC_PI_4;

use bevy::prelude::{
    Bundle, Commands, Component, Entity, EventReader, Query, Res, ResMut, State, Time, Timer, With,
};
use gdnative::api::{CollisionShape2D, KinematicBody2D};
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::PlayingGame;
use gdrust::ecs::engine_sync::resources::PhysicsDelta;
use gdrust::macros::*;
use gdrust::unsafe_functions::{NodeExt, RefExt};

use crate::delect_box::hit_box::HitBox;
use crate::{
    components::{Acceleration, Animation, Friction, Roll, Stats, Velocity},
    delect_box::hurt_box::HurtBox,
};

pub struct SpawnPlayer {
    pub node: Ref<Node>,
}

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
    #[component("HixboxPivot/SwordHitbox")]
    sword_hitbox: HitBox,
    #[component("Hurtbox")]
    hurt_box: HurtBox,
}

/// Add Player Node Event.
/// This event is used to add the player node to the scene.
pub fn add_player_system(mut commands: Commands, mut event: EventReader<SpawnPlayer>) {
    for SpawnPlayer { node } in event.iter() {
        node.expect_safe()
            .expect_node::<CollisionShape2D, &str>("HixboxPivot/SwordHitbox/CollisionShape2D")
            .set_disabled(true);

        commands
            .spawn_bundle(PlayerBundle::new(node.clone()))
            .insert(PlayingGame);
    }
}

/// Player state system.
/// This system is used to determine the player's state.
pub fn player_state_system(
    mut state: ResMut<State<PlayerState>>,
    mut commands: Commands,
    time: Res<Time>,
    mut animation: Query<(Entity, &mut Velocity, &mut Timer), With<Player>>,
) {
    if state.current().eq(&PlayerState::MOVE) {
        let input = Input::godot_singleton();
        if input.is_action_just_pressed("attack", false) {
            state.set(PlayerState::ATTACK).unwrap();
        }
        if input.is_action_just_pressed("roll", false) {
            state.set(PlayerState::ROLL).unwrap();
        }
    }

    for (entity, mut velocity, mut timer) in animation.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(entity).remove::<Timer>();
            state.set(PlayerState::MOVE).unwrap();

            if state.current().eq(&PlayerState::ROLL) {
                (*velocity).value = Vector2::ZERO;
            }
        }
    }
}

/// Player Move System.
/// This system is used to change player's velocity.
pub fn player_move_system(
    delta: Res<PhysicsDelta>,
    mut query0: Query<
        (
            &mut HitBox,
            &Animation,
            &mut Velocity,
            &Acceleration,
            &Friction,
            &mut Roll,
        ),
        With<Player>,
    >,
) {
    for (mut sword_hitbox, animation, mut velocity, acceleration, friction, mut roll) in
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
    }
}

/// Player Attack System.
/// This system is used to attack the player.
pub fn player_attack_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &Animation), With<Player>>,
) {
    for (entity, mut velocity, animation) in query.iter_mut() {
        (*velocity).value = Vector2::ZERO;
        animation.animation_state.expect_safe().travel("Attack");

        commands
            .entity(entity)
            .insert(Timer::from_seconds(0.4, false));
    }
}

/// Player Roll System.
/// This system is used to roll the player.
pub fn player_roll_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &Animation, &Roll), With<Player>>,
) {
    for (entity, mut velocity, animation, roll) in query.iter_mut() {
        (*velocity).value = roll.roll_velocity * roll.roll_speed;
        animation.animation_state.expect_safe().travel("Roll");

        commands
            .entity(entity)
            .insert(Timer::from_seconds(0.5, false));
    }
}

/// Player Move System.
/// This system is used to move the player.
pub fn player_movement_system(mut query: Query<(&mut Velocity, &Player)>) {
    for (mut velocity, player) in query.iter_mut() {
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
