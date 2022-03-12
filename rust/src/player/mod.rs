pub(crate) use crate::player::player::*;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use gdrust::ecs::engine_sync::stages::SyncStages;

mod player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_state_system)
            .add_system(player_timer_system)
            .add_system(player_move_system)
            .add_system_set(
                SystemSet::new()
                    .with_system(attack_player_system)
                    .with_system(attack_player_exit_system),
            )
            .add_system_to_stage(
                SyncStages::UpdateBevyPhysics,
                player_movement_system.label("player_movement"),
            )
            .add_system_to_stage(
                SyncStages::UpdateBevyPhysics,
                player_no_health_system.after("player_movement"),
            );
    }
}
