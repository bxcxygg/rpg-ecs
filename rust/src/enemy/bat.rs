use crate::components::{Acceleration, Friction, Knockback, Stats, Velocity};
use crate::delect_box::hit_box::HitBox;
use crate::delect_box::hurt_box::{HurtBox, HIT_EFFECT_LENGTH};
use crate::delect_box::soft_collision::SoftCollision;
use crate::effect::{add_effect, Effect};
use crate::enemy::wander_controller::WanderTimer;
use crate::enemy::DelectionZone;
use crate::player::{Player, PlayerAttacking};
use crate::{with_world, WanderController};
use bevy::prelude::{Bundle, Commands, Component, Entity, Query, Res, Timer, With, Without};
use defaults::Defaults;
use gdnative::api::{AnimatedSprite, Area2D, KinematicBody2D};
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::{GodotObjRef, PlayingGame};
use gdrust::ecs::engine_sync::resources::PhysicsDelta;
use gdrust::macros::*;
use gdrust::unsafe_functions::{NodeExt, NodeTreeExt, RefExt, ResourceLoaderExt};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::f64::consts::FRAC_PI_4;
use std::ops::Range;
use std::time::Duration;

const ENEMY_DEATH_LENGTH: f32 = 9. / 15.;

#[derive(Component, Clone, Hash, Eq, PartialEq, Default, Copy)]
pub enum BatState {
    #[default]
    IDLE,
    WANDER,
    CHASE,
}

#[derive(Component, Defaults, Copy, Clone)]
pub struct Bat {
    #[def = "KinematicBody2D::new().into_shared()"]
    pub owner: Ref<KinematicBody2D>,
}

#[derive(Component, Default, Clone)]
pub struct BatDeadEffect(pub Effect);

#[derive(Component, Default, Clone)]
pub struct BatKnockback(pub Knockback);

#[gdrust(extends = KinematicBody2D)]
#[derive(Bundle, Default, Clone)]
pub struct BatBundle {
    #[default(Bat{ owner: _owner.claim() })]
    pub bat: Bat,
    pub state: BatState,
    #[export]
    pub stats: Stats,
    pub knock: BatKnockback,
    pub velocity: Velocity,
    #[export]
    pub acceleration: Acceleration,
    #[export]
    pub friction: Friction,
}
#[methods]
impl BatBundle {
    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        with_world(|w| {
            let effect = Effect::new(
                ResourceLoader::godot_singleton()
                    .expect_load_scene("res://scenes/effect/EnemyDeathEffect.tscn"),
            );

            w.spawn()
                .insert_bundle(self.clone())
                .insert(
                    owner
                        .expect_instance::<HitBox>("Hitbox")
                        .map(|h, _| h.clone())
                        .unwrap(),
                )
                .insert(
                    owner
                        .expect_instance::<HurtBox>("Hurtbox")
                        .map(|h, _| h.clone())
                        .unwrap(),
                )
                .insert(
                    owner
                        .expect_instance::<SoftCollision>("SoftCollision")
                        .map(|h, _| h.clone())
                        .unwrap(),
                )
                .insert(
                    owner
                        .expect_instance::<WanderController>("WanderController")
                        .map(|h, _| h.clone())
                        .unwrap(),
                )
                .insert(WanderTimer(Timer::from_seconds(3., false)))
                .insert(GodotObjRef::new(
                    owner.expect_node::<AnimatedSprite>("Sprite").claim(),
                ))
                .insert(BatDeadEffect(effect))
                .insert(DelectionZone {
                    owner: owner.expect_node::<Area2D>("Zone").claim(),
                    player: None,
                })
                .insert(PlayingGame);
        })
    }
}

pub fn bat_system(
    delta: Res<PhysicsDelta>,
    mut bat: Query<(
        &mut Velocity,
        &Friction,
        &DelectionZone,
        &mut BatState,
        &Bat,
        &Acceleration,
        &GodotObjRef<AnimatedSprite>,
        &mut WanderTimer,
        &WanderController,
    )>,
) {
    for (
        mut velocity,
        friction,
        delect_zone,
        mut state,
        bat,
        acceleration,
        sprite,
        mut timer,
        wander_controller,
    ) in bat.iter_mut()
    {
        match *state {
            BatState::IDLE => {
                velocity.velocity =
                    velocity.move_toward(Vector2::ZERO, friction.friction * delta.value);
                bat_idle_or_wander(delect_zone, &mut *state, &mut timer.0);
            }
            BatState::CHASE => bat_chase(
                &mut *velocity,
                acceleration,
                delect_zone,
                bat,
                &mut *state,
                sprite,
                &delta,
            ),
            BatState::WANDER => {
                bat_idle_or_wander(delect_zone, &mut *state, &mut timer.0);
                let direction = bat
                    .owner
                    .expect_safe()
                    .global_position()
                    .direction_to(wander_controller.target_position);

                velocity.velocity = velocity.move_toward(
                    direction * acceleration.max_speed,
                    acceleration.acceleration * delta.value,
                );
            }
        }
    }
}

/// Bat Idle State System.
/// This system is responsible for the bat's idle state.
pub fn bat_idle_or_wander(delect_zone: &DelectionZone, state: &mut BatState, timer: &mut Timer) {
    if delect_zone.player.is_some() {
        *state = BatState::CHASE;
        ()
    }

    if timer.percent_left() == 0. {
        let mut rng = rand::thread_rng();
        let mut state_list: [BatState; 2] = [BatState::IDLE, BatState::WANDER];
        state_list.shuffle(&mut rng);
        *state = state_list[0];

        timer.set_duration(Duration::from_secs_f32(
            rng.gen_range(Range { start: 1., end: 3. }),
        ));
        timer.reset();
    }
}

/// Bat Chase State System.
/// This system is responsible for the bat's chase state.
/// The bat will chase the player.
/// If the player is out of the bat's delection zone, the bat will go back to idle.
/// If the player is in the bat's delection zone, the bat will chase the player.
pub fn bat_chase(
    velocity: &mut Velocity,
    acceleration: &Acceleration,
    delect_zone: &DelectionZone,
    bat: &Bat,
    state: &mut BatState,
    sprite: &GodotObjRef<AnimatedSprite>,
    delta: &PhysicsDelta,
) {
    if let Some(player) = delect_zone.player {
        let player_pos = player.expect_safe().global_position();
        let direction = bat
            .owner
            .expect_safe()
            .global_position()
            .direction_to(player_pos);

        velocity.velocity = velocity.move_toward(
            direction * acceleration.max_speed,
            acceleration.acceleration * delta.value,
        );
    } else {
        *state = BatState::IDLE;
        velocity.velocity = Vector2::ZERO;
    }

    sprite.expect_safe().set_flip_v(velocity.x < 0.0);
}

/// Bat Move System.
/// This system is responsible for the bat's movement.
/// The bat will move according to the velocity.
pub fn bat_move_system(
    mut query: Query<(&mut Velocity, &mut BatKnockback, &Bat, &SoftCollision)>,
    delta: Res<PhysicsDelta>,
) {
    for (mut velocity, mut knockback, bat, soft_collision) in query.iter_mut() {
        let bat = bat.owner.expect_safe();
        knockback.0.vector = knockback
            .0
            .vector
            .move_toward(Vector2::ZERO, 200. * delta.value);
        bat.move_and_slide(knockback.0.vector, Vector2::ZERO, false, 4, FRAC_PI_4, true);

        // let soft_collision_area = soft_collision.owner.expect_safe();
        // if !soft_collision_area.get_overlapping_areas().is_empty() {
        velocity.velocity += soft_collision.input_vector;
        // }

        velocity.velocity =
            bat.move_and_slide(velocity.velocity, Vector2::ZERO, false, 4, FRAC_PI_4, true);
    }
}

/// Attack Bat System.
pub fn attack_bat_system(
    mut commands: Commands,
    mut bat: Query<
        (
            Entity,
            &HurtBox,
            &mut BatKnockback,
            &mut Stats,
            &Bat,
            &BatDeadEffect,
            &HitBox,
        ),
        Without<PlayerAttacking>,
    >,
    hitbox: Query<&HitBox, With<Player>>,
) {
    for (entity, hurtbox, mut knockback, mut stats, bat, dead_effect, bat_hitbox) in bat.iter_mut()
    {
        let hurtbox_area = hurtbox.owner.expect_safe();
        let bat = bat.owner.expect_safe();

        for hitbox in hitbox.iter() {
            let hitbox_area = hitbox.owner.expect_safe();
            if hurtbox_area.overlaps_area(hitbox_area) {
                stats.health -= bat_hitbox.damage;
                knockback.0.vector = hitbox.knockback * 120.;

                let positon = bat.global_position();
                let parent = bat.expect_tree().current_scene().unwrap().expect_safe();

                commands.entity(entity).insert(PlayerAttacking);

                // spawn the effect
                let entity_commands = commands.spawn();
                add_effect(
                    entity_commands,
                    hurtbox.effect.clone(),
                    HIT_EFFECT_LENGTH,
                    positon,
                    parent,
                );

                if stats.health <= 0 {
                    let entity_commands = commands.spawn();
                    add_effect(
                        entity_commands,
                        dead_effect.0.effect.clone(),
                        ENEMY_DEATH_LENGTH,
                        positon,
                        parent,
                    );

                    commands.entity(entity).despawn();
                    bat.queue_free();
                }
            }
        }
    }
}

/// Attack Bat System.
pub fn attack_exit_bat_system(
    mut commands: Commands,
    mut enmemy: Query<(Entity, &HurtBox), With<PlayerAttacking>>,
    hitbox: Query<&HitBox, With<Player>>,
) {
    for (entity, hurtbox) in enmemy.iter_mut() {
        let hurtbox_area = hurtbox.owner.expect_safe();

        for hitbox in hitbox.iter() {
            let hitbox_area = hitbox.owner.expect_safe();
            if !hurtbox_area.overlaps_area(hitbox_area) {
                commands.entity(entity).remove::<PlayerAttacking>();
            }
        }
    }
}
