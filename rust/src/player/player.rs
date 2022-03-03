use bevy::prelude::{Bundle, Commands, Component, EventReader, StageLabel};
use gdnative::api::KinematicBody2D;
use gdnative::prelude::{Node, Ref, Vector2};
use gdrust::ecs::engine_sync::events::SpawnNode;
use gdrust::gdrust_macros::{gdbundle, gdcomponent};
use gdrust::unsafe_functions::{NodeExt, RefExt};

use crate::delect_box::hit_box::HitBoxPosition;
use crate::{
    components::{Acceleration, AnimationTreeComponent, Friction, Roll, Stats, Velocity},
    delect_box::hurt_box::HurtBox,
};

/// player state.
/// This is the state of the player.
/// It is used to determine the player's state.
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone, StageLabel)]
pub enum PlayerStages {
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
    #[value(Velocity{value: Vector2::ZERO})]
    velocity: Velocity,
    #[component("Stats")]
    stats: Stats,
    #[component("AnimationTree")]
    animation_tree: AnimationTreeComponent,
    #[component("HixboxPivot")]
    sword_hitbox: HitBoxPosition,
    #[component("Hurtbox")]
    hurt_box: HurtBox,
}

/// Add Player Node Event.
/// This event is used to add the player node to the scene.
pub fn add_player_system(mut commands: Commands, mut event: EventReader<SpawnNode>) {
    for SpawnNode { node, name } in event.iter() {
        if name != "Player" {
            continue;
        }

        commands.spawn_bundle(PlayerBundle::new(node.clone()));
    }
}
