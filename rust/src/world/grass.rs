use crate::delect_box::hit_box::HitBox;
use bevy::prelude::{Commands, Component, Entity, EventReader, Query, With};
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::{GodotObjRef, PlayingGame};
use gdrust::macros::gdcomponent;
use gdrust::unsafe_functions::{NodeExt, RefExt, ResourceLoaderExt};

use crate::delect_box::hurt_box::HurtBox;
use crate::effect::grass_effect::add_effect;
use crate::player::Player;

const GRASS_EFFECT_LEN: f32 = 4. / 15.;

pub struct SpawnGrass {
    pub node: Ref<Node>,
}

#[gdcomponent(extends = Node2D)]
pub struct Grass {
    #[node]
    node: Ref<Node2D>,
}

/// Add grass to the scene.
/// use `SpawnGrass` event to add grass to the scene.
pub fn add_grass_system(mut commands: Commands, mut event: EventReader<SpawnGrass>) {
    for SpawnGrass { node } in event.iter() {
        let node = node.expect_safe();

        let effect = ResourceLoader::godot_singleton()
            .expect_load_scene("res://scenes/effect/GrassEffect.tscn");
        commands
            .spawn()
            .insert(Grass::new(node.claim()))
            .insert(HurtBox::new(
                node.expect_node::<Node, &str>("HurtBox").claim(),
            ))
            .insert(GodotObjRef::new(effect))
            .insert(PlayingGame);
    }
}

/// Kill grass when it is hit by a player.
pub fn kill_grass_system(
    mut commands: Commands,
    q0: Query<&HitBox, With<Player>>,
    q1: Query<(Entity, &Grass, &HurtBox, &GodotObjRef<PackedScene>)>,
) {
    for hitbox in q0.iter() {
        let hitbox_area = hitbox.node.expect_safe();

        for (entity, grass, hurtbox, effect) in q1.iter() {
            let grass = grass.node.expect_safe();
            let hurtbox = hurtbox.node.expect_safe();

            // process collision
            if hitbox_area.overlaps_area(hurtbox) {
                // spawn the effect
                let entity_commands = commands.spawn();
                add_effect(
                    entity_commands,
                    effect.expect_safe().claim(),
                    GRASS_EFFECT_LEN,
                    grass.global_position(),
                    grass.expect_tree().current_scene().unwrap().expect_safe(),
                );

                // remove the grass
                commands.entity(entity).despawn();
                grass.queue_free();
            }
        }
    }
}
