use crate::delect_box::hit_box::HitBox;
use bevy::prelude::{Commands, Component, Entity, Query, Res, With};
use gdnative::prelude::*;
use gdrust::ecs::app::with_world;
use gdrust::ecs::engine_sync::components::PlayingGame;
use gdrust::macros::*;
use gdrust::unsafe_functions::{NodeExt, NodeTreeExt, RefExt};

use crate::delect_box::hurt_box::HurtBox;
use crate::effect::{add_effect, GrassEffect};
use crate::player::Player;

#[gdrust(extends = Node2D)]
#[derive(Component, Clone)]
pub struct Grass {
    #[default(_owner.claim())]
    pub owner: Ref<Node2D>,
}
#[methods]
impl Grass {
    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        with_world(|w| {
            w.spawn()
                .insert(self.clone())
                .insert(
                    owner
                        .expect_instance::<HurtBox>("HurtBox")
                        .map(|h, _| h.clone())
                        .unwrap(),
                )
                .insert(PlayingGame);
        })
    }
}

/// Kill grass when it is hit by a player.
pub fn kill_grass_system(
    mut commands: Commands,
    grass_effect: Res<GrassEffect>,
    player: Query<&HitBox, With<Player>>,
    grass: Query<(Entity, &Grass, &HurtBox)>,
) {
    for hitbox in player.iter() {
        let hitbox_area = hitbox.owner.expect_safe();

        for (entity, grass, hurtbox) in grass.iter() {
            let grass_ref = grass.owner.expect_safe();
            let hurtbox = hurtbox.owner.expect_safe();

            // process collision
            if hitbox_area.overlaps_area(hurtbox) {
                // spawn the effect
                add_effect(
                    &mut commands,
                    &grass_effect.effect,
                    grass_ref.global_position(),
                    grass_ref
                        .expect_tree()
                        .current_scene()
                        .unwrap()
                        .expect_safe(),
                );

                // remove the grass
                commands.entity(entity).despawn();
                grass_ref.queue_free();
            }
        }
    }
}
