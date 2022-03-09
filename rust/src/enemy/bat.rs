use crate::components::{Acceleration, AnimationSprite, Friction, Stats, Velocity};
use crate::delect_box::hit_box::HitBox;
use crate::delect_box::hurt_box::{HurtBox, HIT_EFFECT_LENGTH};
use crate::effect::grass_effect::add_effect;
use crate::enemy::{DelectionZone, Knockback};
use crate::player::Player;
use bevy::prelude::{Commands, Component, Entity, EventReader, Query, Res, With};
use gdnative::api::KinematicBody2D;
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::GodotObjRef;
use gdrust::ecs::engine_sync::resources::PhysicsDelta;
use gdrust::macros::gdcomponent;
use gdrust::unsafe_functions::{NodeExt, RefExt, ResourceLoaderExt};
use std::f64::consts::FRAC_PI_4;

const ENEMY_DEATH_LENGTH: f32 = 9. / 15.;

pub struct SpawnBat {
    pub node: Ref<Node>,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
pub enum BatState {
    IDLE,
    WANDER,
    CHASE,
}

#[gdcomponent(extends = KinematicBody2D)]
pub struct Bat {
    #[node]
    pub node: Ref<KinematicBody2D>,
    #[value(BatState::IDLE)]
    pub state: BatState,
}

/// Add Bat component to the entity
pub fn add_bat_system(mut commands: Commands, mut event: EventReader<SpawnBat>) {
    for SpawnBat { node } in event.iter() {
        let node = node.expect_safe();
        let effect = ResourceLoader::godot_singleton()
            .expect_load_scene("res://scenes/effect/EnemyDeathEffect.tscn");

        commands
            .spawn()
            .insert(Bat::new(node.claim()))
            .insert(Stats::new(node.expect_node::<Node, &str>("Stats").claim()))
            .insert(DelectionZone::new(
                node.expect_node::<Node, &str>("Zone").claim(),
            ))
            .insert(GodotObjRef::new(effect))
            .insert(HurtBox::new(
                node.expect_node::<Node, &str>("Hurtbox").claim(),
            ))
            .insert(AnimationSprite::new(
                node.expect_node::<Node, &str>("Sprite").claim(),
            ))
            .insert(Knockback {
                knockback: Vector2::ZERO,
            })
            .insert(Velocity::new(Vector2::ZERO))
            .insert(Acceleration::new(
                node.expect_node::<Node, &str>("Acceleration").claim(),
            ))
            .insert(Friction::new(
                node.expect_node::<Node, &str>("Friction").claim(),
            ));
    }
}

/// Bat Idle State System.
/// This system is responsible for the bat's idle state.
pub fn bat_idle_system(
    mut bat: Query<(&mut Velocity, &Friction, &DelectionZone, &mut Bat)>,
    delta: Res<PhysicsDelta>,
) {
    for (mut velocity, friction, delection_zone, mut bat) in bat.iter_mut() {
        if bat.state == BatState::IDLE {
            (*velocity).value = velocity.move_toward(Vector2::ZERO, friction.value * delta.value);

            if delection_zone.player.is_some() {
                bat.state = BatState::CHASE;
            }
        }
    }
}

/// Bat Chase State System.
/// This system is responsible for the bat's chase state.
/// The bat will chase the player.
/// If the player is out of the bat's delection zone, the bat will go back to idle.
/// If the player is in the bat's delection zone, the bat will chase the player.
pub fn bat_chase_system(
    mut bat: Query<(
        &mut Velocity,
        &Acceleration,
        &DelectionZone,
        &mut Bat,
        &AnimationSprite,
    )>,
    delta: Res<PhysicsDelta>,
) {
    for (mut velocity, acceleration, delection_zone, mut bat, sprite) in bat.iter_mut() {
        if bat.state == BatState::CHASE {
            if let Some(player) = delection_zone.player {
                let player_pos = player.expect_safe().global_position();
                let direction =
                    (player_pos - bat.node.expect_safe().global_position()).normalized();

                (*velocity).value = velocity.move_toward(
                    direction * acceleration.max_speed,
                    acceleration.acceleration * delta.value,
                );
            } else {
                bat.state = BatState::IDLE;
                (*velocity).value = Vector2::ZERO;
            }

            sprite.node.expect_safe().set_flip_v(velocity.x < 0.0);
        }
    }
}

/// Bat Move System.
/// This system is responsible for the bat's movement.
/// The bat will move according to the velocity.
pub fn bat_move_system(
    mut query: Query<(&mut Velocity, &mut Knockback, &Bat)>,
    delta: Res<PhysicsDelta>,
) {
    for (mut velocity, mut knockback, bat) in query.iter_mut() {
        let bat = bat.node.expect_safe();
        knockback.knockback = knockback
            .knockback
            .move_toward(Vector2::ZERO, 200. * delta.value);
        bat.move_and_slide(
            knockback.knockback,
            Vector2::ZERO,
            false,
            4,
            FRAC_PI_4,
            true,
        );

        (*velocity).value =
            bat.move_and_slide((*velocity).value, Vector2::ZERO, false, 4, FRAC_PI_4, true);
    }
}

/// Attack Bat System.
pub fn attack_bat_system(
    mut commands: Commands,
    mut bat: Query<(
        Entity,
        &HurtBox,
        &mut Knockback,
        &mut Stats,
        &Bat,
        &GodotObjRef<PackedScene>,
    )>,
    hitbox: Query<&HitBox, With<Player>>,
) {
    for (entity, hurtbox, mut knockback, mut stats, bat, effect) in bat.iter_mut() {
        let hurtbox_area = hurtbox.node.expect_safe();
        let bat = bat.node.expect_safe();

        for hitbox in hitbox.iter() {
            let hitbox_area = hitbox.node.expect_safe();
            if hurtbox_area.overlaps_area(hitbox_area) {
                stats.health -= hitbox.damage;
                knockback.knockback = hitbox.knockback_vector * 120.;

                let positon = bat.global_position();
                let parent = bat.expect_tree().current_scene().unwrap().expect_safe();

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
                        effect.expect_safe().claim(),
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
