use crate::delect_box::hit_box::HitBoxPosition;
use crate::delect_box::hurt_box::HurtBox;
use crate::effect::grass_effect::Effect;
use crate::player::Player;
use bevy::prelude::{Commands, Component, Entity, EventReader, Query, With};
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::{GodotObjRef, PlayingGame};
use gdrust::macros::gdcomponent;
use gdrust::unsafe_functions::{NodeExt, RefExt, ResourceLoaderExt};

pub struct SpawnGrass {
    pub node: Ref<Node>,
}

pub struct AreaIntoGrass {
    pub grass: Ref<Node2D>,
    pub area: Ref<Area2D>,
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

pub fn kill_grass_system(
    mut commands: Commands,
    grasses: Query<(Entity, &Grass, &GodotObjRef<PackedScene>)>,
    mut event: EventReader<AreaIntoGrass>,
) {
    for AreaIntoGrass { grass, area: _area } in event.iter() {
        for (entity, grass_item, effect) in grasses.iter() {
            let grass = grass.expect_safe();
            let grass_item = grass_item.node.expect_safe();

            // process collision
            if grass.get_instance_id() == grass_item.get_instance_id() {
                // spawn the effect
                commands.spawn().insert(Effect::new(
                    grass.global_position(),
                    effect.expect_safe().claim(),
                    grass.expect_tree().current_scene().unwrap().expect_safe(),
                ));
                // remove the grass
                commands.entity(entity).despawn();
                grass.queue_free();
            }
        }
    }
}
