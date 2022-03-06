pub(crate) use crate::player::player::*;
use bevy::prelude::{App, Plugin, SystemSet};
use gdrust::ecs::engine_sync::stages::SyncStages;

mod player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPlayer>()
            .add_state_to_stage(SyncStages::UpdateBevy, PlayerState::MOVE)
            .add_system(add_player_system)
            .add_system_set_to_stage(
                SyncStages::UpdateBevy,
                SystemSet::on_update(PlayerState::MOVE)
                    .with_system(player_move_state_system)
                    .with_system(player_move_system),
            )
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, player_movement_system)
            .add_system_set_to_stage(
                SyncStages::UpdateBevy,
                SystemSet::on_enter(PlayerState::ATTACK).with_system(player_attack_system),
            )
            .add_system_set_to_stage(
                SyncStages::UpdateBevy,
                SystemSet::on_update(PlayerState::ATTACK).with_system(player_attack_state_system),
            )
            .add_system_set_to_stage(
                SyncStages::UpdateBevy,
                SystemSet::on_enter(PlayerState::ROLL).with_system(player_roll_system),
            )
            .add_system_set_to_stage(
                SyncStages::UpdateBevy,
                SystemSet::on_update(PlayerState::ROLL).with_system(player_roll_state_system),
            );
    }
}
