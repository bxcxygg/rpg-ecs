use bevy::prelude::{Bundle, Commands, Component, EventReader, StageLabel};

use crate::{
    components::{Acceleration, AnimationTreeComponent, Friction, Roll, Stats, Velocity},
    delect_box::{hit_box::HitBox, hurt_box::HurtBox},
    engine_sync::events::SpawnNode,
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

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    acceleration: Acceleration,
    friction: Friction,
    roll: Roll,
    velocity: Velocity,
    stats: Stats,
    animation_tree: AnimationTreeComponent,
    sword_hitbox: HitBox,
    hurt_box: HurtBox,
}

/// Add Player Node Event.
/// This event is used to add the player node to the scene.
pub fn add_player_system(mut commands: Commands, mut event: EventReader<SpawnNode>) {
    for event in event.iter() {}
}
