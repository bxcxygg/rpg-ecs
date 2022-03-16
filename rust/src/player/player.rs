use std::f64::consts::FRAC_PI_4;

use bevy::prelude::{
    Bundle, Commands, Component, Entity, EventWriter, Query, Res, Time, Timer, With, Without,
};
use defaults::Defaults;
use gdnative::api::{
    AnimationNodeStateMachinePlayback, AnimationPlayer, AnimationTree, CollisionShape2D,
    KinematicBody2D,
};
use gdnative::prelude::*;
use gdrust::ecs::app::with_world;
use gdrust::ecs::engine_sync::components::{GodotObjRef, PlayingGame};
use gdrust::ecs::engine_sync::resources::PhysicsDelta;
use gdrust::macros::*;
use gdrust::unsafe_functions::{NodeExt, RefExt};

use crate::components::{Acceleration, Friction, Roll, Stats, Velocity};
use crate::delect_box::hit_box::HitBox;
use crate::delect_box::hurt_box::HurtBox;
use crate::enemy::bat::Bat;
use crate::world::health::ChangeHealth;

const ATTACK_ANIMATION_LEN: f32 = 0.4;
const ROLL_ANIMATION_LEN: f32 = 0.5;

/// player state.
/// This is the state of the player.
/// It is used to determine the player's state.
#[derive(Component, Defaults, Clone, Copy, Eq, PartialEq)]
#[def = "MOVE"]
pub enum PlayerState {
    MOVE,
    ATTACK,
    ROLL,
}

#[derive(Component, Default, Clone, Copy)]
pub struct PlayerBeenAttack;
#[derive(Component, Default, Clone, Copy)]
pub struct PlayerAttacking;

/// Player Component.
/// This is the component of the player.
#[derive(Component, Clone, Copy)]
pub struct Player {
    pub owner: Ref<KinematicBody2D>,
}

/// Player bundle.
#[gdrust(extends = KinematicBody2D)]
#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    #[default(Player{owner: _owner.claim()})]
    player: Player,
    state: PlayerState,
    #[export]
    stats: Stats,
    velocity: Velocity,
    #[export]
    acceleration: Acceleration,
    #[export]
    friction: Friction,
    #[export]
    roll: Roll,
}

#[methods]
impl PlayerBundle {
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
                .insert_bundle(self.clone())
                .insert(
                    owner
                        .expect_instance::<HitBox>("HixboxPivot/SwordHitbox")
                        .map(|h, _| h.clone())
                        .unwrap(),
                )
                .insert(
                    owner
                        .expect_instance::<HurtBox>("Hurtbox")
                        .map(|h, _| h.clone())
                        .unwrap(),
                )
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
pub fn player_state_system(mut player: Query<&mut PlayerState, With<Player>>) {
    let input = Input::godot_singleton();

    for mut state in player.iter_mut() {
        if *state == PlayerState::MOVE {
            if input.is_action_just_pressed("attack", false) {
                *state = PlayerState::ATTACK;
            }
            if input.is_action_just_pressed("roll", false) {
                *state = PlayerState::ROLL;
            }
        }
    }
}

pub fn player_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<(Entity, &mut Velocity, &mut Timer, &mut PlayerState), With<Player>>,
) {
    for (entity, mut velocity, mut timer, mut state) in player.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if *state == PlayerState::ROLL {
                velocity.velocity = Vector2::ZERO;
            }
            commands.entity(entity).remove::<Timer>();
            *state = PlayerState::MOVE;
        }
    }
}

/// Player Move System.
/// This system is used to change player's velocity.
pub fn player_move_system(
    mut commands: Commands,
    delta: Res<PhysicsDelta>,
    mut player: Query<
        (
            Entity,
            &GodotObjRef<AnimationTree>,
            &GodotObjRef<AnimationNodeStateMachinePlayback>,
            &mut Velocity,
            &mut HitBox,
            &Acceleration,
            &Friction,
            &mut Roll,
            &PlayerState,
            Option<&Timer>,
        ),
        With<Player>,
    >,
) {
    for (
        entity,
        animation_tree,
        animation_state,
        mut velocity,
        mut hitbox,
        acceleration,
        friction,
        mut roll,
        state,
        timer,
    ) in player.iter_mut()
    {
        match *state {
            PlayerState::MOVE => player_move(
                animation_tree.expect_safe(),
                animation_state.expect_safe(),
                &mut *hitbox,
                &mut *roll,
                &mut *velocity,
                acceleration,
                friction,
                &delta,
            ),
            PlayerState::ATTACK => {
                if timer.is_none() {
                    player_attack(
                        &mut commands,
                        &entity,
                        &mut *velocity,
                        animation_state.expect_safe(),
                    )
                }
            }
            PlayerState::ROLL => {
                if timer.is_none() {
                    player_roll(
                        &mut commands,
                        &entity,
                        &mut *velocity,
                        animation_state.expect_safe(),
                        &mut *roll,
                    )
                }
            }
        }
    }
}

fn player_move(
    animation_tree: TRef<AnimationTree>,
    animation_state: TRef<AnimationNodeStateMachinePlayback>,
    hitbox: &mut HitBox,
    roll: &mut Roll,
    velocity: &mut Velocity,
    acceleration: &Acceleration,
    friction: &Friction,
    delta: &PhysicsDelta,
) {
    let input = Input::godot_singleton();
    let mut input_vector = Vector2::new(
        input.get_action_strength("ui_right", false) as f32
            - input.get_action_strength("ui_left", false) as f32,
        input.get_action_strength("ui_down", false) as f32
            - input.get_action_strength("ui_up", false) as f32,
    );

    if input_vector != Vector2::ZERO {
        input_vector = input_vector.normalized();
        hitbox.knockback = input_vector;
        roll.roll_velocity = input_vector;

        animation_tree.set("parameters/Idle/blend_position", input_vector);
        animation_tree.set("parameters/Run/blend_position", input_vector);
        animation_tree.set("parameters/Attack/blend_position", input_vector);
        animation_tree.set("parameters/Roll/blend_position", input_vector);

        animation_state.travel("Run");

        velocity.velocity = velocity.move_toward(
            input_vector * acceleration.max_speed,
            acceleration.acceleration * delta.value,
        );
    } else {
        animation_state.travel("Idle");
        velocity.velocity = velocity.move_toward(Vector2::ZERO, friction.friction * delta.value);
    }
}

/// Player Attack System.
/// This system is used to attack the player.
pub fn player_attack(
    commands: &mut Commands,
    entity: &Entity,
    velocity: &mut Velocity,
    animation_state: TRef<AnimationNodeStateMachinePlayback>,
) {
    velocity.velocity = Vector2::ZERO;
    animation_state.travel("Attack");

    commands
        .entity(*entity)
        .insert(Timer::from_seconds(ATTACK_ANIMATION_LEN, false));
}

/// Player Roll System.
/// This system is used to roll the player.
pub fn player_roll(
    commands: &mut Commands,
    entity: &Entity,
    velocity: &mut Velocity,
    animation_state: TRef<AnimationNodeStateMachinePlayback>,
    roll: &Roll,
) {
    velocity.velocity = roll.roll_velocity * roll.roll_speed;
    animation_state.travel("Roll");

    commands
        .entity(*entity)
        .insert(Timer::from_seconds(ROLL_ANIMATION_LEN, false));
}

/// Player Move System.
/// This system is used to move the player.
pub fn player_movement_system(mut query: Query<(&mut Velocity, &Player)>) {
    for (mut velocity, player) in query.iter_mut() {
        velocity.velocity = player.owner.expect_safe().move_and_slide(
            velocity.velocity,
            Vector2::ZERO,
            false,
            4,
            FRAC_PI_4,
            true,
        );
    }
}

pub fn attack_player_system(
    mut commands: Commands,
    mut event: EventWriter<ChangeHealth>,
    mut player: Query<(&mut Stats, &HurtBox), With<Player>>,
    enemy: Query<(Entity, &HitBox), (With<Bat>, Without<PlayerBeenAttack>)>,
) {
    for (mut stats, hurtbox) in player.iter_mut() {
        let hurtbox = hurtbox.owner.expect_safe();

        for (bat_entity, hitbox) in enemy.iter() {
            let hitbox = hitbox.owner.expect_safe();

            if hurtbox.overlaps_area(hitbox) {
                stats.health -= 1;

                event.send(ChangeHealth {
                    health: stats.health,
                });

                commands.entity(bat_entity).insert(PlayerBeenAttack);
            }
        }
    }
}

pub fn attack_player_exit_system(
    mut commands: Commands,
    player: Query<&HurtBox, With<Player>>,
    enemy: Query<(Entity, &HitBox), (With<Bat>, With<PlayerBeenAttack>)>,
) {
    for hurtbox in player.iter() {
        let hurtbox = hurtbox.owner.expect_safe();

        for (entity, hitbox) in enemy.iter() {
            let hitbox = hitbox.owner.expect_safe();

            if !hurtbox.overlaps_area(hitbox) {
                commands.entity(entity).remove::<PlayerBeenAttack>();
            }
        }
    }
}

pub fn player_no_health_system(
    mut commands: Commands,
    // mut game_over: ResMut<Option<GameOver>>,
    player: Query<(Entity, &mut Stats, &Player)>,
) {
    for (entity, stats, player) in player.iter() {
        let player = player.owner.expect_safe();
        if stats.health == 0 {
            commands.entity(entity).despawn();
            player.queue_free();

            // *game_over = Some(GameOver::Lose);
        }
    }
}
