use crate::player::player::{PlayerAttackAnimationFinished, PlayerRollAnimationFinished};
use bevy::app::EventWriter;
use bevy::prelude::EventReader;
use gdrust::ecs::engine_sync::events::SpawnSignal;

pub fn signal_event_system(
    mut event: EventReader<SpawnSignal>,
    mut out0: EventWriter<PlayerAttackAnimationFinished>,
    mut out1: EventWriter<PlayerRollAnimationFinished>,
) {
    for SpawnSignal { name, vars: _vars } in event.iter() {
        match name.as_str() {
            "attack_animation_finished" => {
                out0.send(PlayerAttackAnimationFinished {});
            }
            "roll_animation_finished" => {
                out1.send(PlayerRollAnimationFinished {});
            }
            _ => (),
        }
    }
}
