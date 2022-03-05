use crate::events::signal_event_system;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::{App, Plugin, PluginGroup, SystemSet, SystemStage};
use gdrust::ecs::engine_sync::stages::SyncStages;

use crate::player::player::{
    add_player_system, player_attack_system, player_move_system, player_roll_system,
    player_state_system, PlayerAttackAnimationFinished, PlayerRollAnimationFinished, PlayerState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttackAnimationFinished>()
            .add_event::<PlayerRollAnimationFinished>()
            .add_system(signal_event_system)
            .add_state_to_stage(SyncStages::UpdateBevyPhysics, PlayerState::MOVE)
            .add_system(add_player_system)
            .add_system(player_state_system)
            .add_system_set_to_stage(
                SyncStages::UpdateBevyPhysics,
                SystemSet::on_update(PlayerState::MOVE).with_system(player_move_system),
            )
            .add_system_set_to_stage(
                SyncStages::UpdateBevyPhysics,
                SystemSet::on_update(PlayerState::ATTACK).with_system(player_attack_system),
            )
            .add_system_set_to_stage(
                SyncStages::UpdateBevyPhysics,
                SystemSet::on_update(PlayerState::ROLL).with_system(player_roll_system),
            );
    }
}

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(GamePlugin);
    }
}
