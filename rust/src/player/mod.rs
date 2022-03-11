pub(crate) use crate::player::player::*;
use bevy::prelude::{App, Plugin};
use gdrust::ecs::engine_sync::stages::SyncStages;

mod player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_state_system)
            .add_system(player_timer_system)
            .add_system(player_move_system)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, player_movement_system);
    }
}
