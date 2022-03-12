use crate::enemy::bat::{attack_bat_system, attack_exit_bat_system, bat_move_system, bat_system};
use crate::player::Player;
use bevy::app::Plugin;
use bevy::prelude::{Component, ParallelSystemDescriptorCoercion, Query};
use defaults::Defaults;
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::stages::SyncStages;
use gdrust::unsafe_functions::RefExt;

pub mod bat;

#[derive(Component, Defaults, Copy, Clone)]
pub struct DelectionZone {
    #[def = "Area2D::new().into_shared()"]
    pub owner: Ref<Area2D>,
    pub player: Option<Ref<KinematicBody2D>>,
}

fn zone_system(mut zone: Query<&mut DelectionZone>, player: Query<&Player>) {
    for mut zone in zone.iter_mut() {
        let zone_area = zone.owner.expect_safe();

        if player.is_empty() {
            zone.player = None;
        }

        for player in player.iter() {
            let player = player.owner.expect_safe();
            if zone_area.overlaps_body(player) {
                zone.player = Some(player.claim());
            } else if zone.player.is_some() {
                zone.player = None;
            }
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system(zone_system.label("zone_system"))
            .add_system(bat_system.after("zone_system"))
            .add_system(attack_exit_bat_system)
            .add_system(attack_bat_system)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, bat_move_system);
    }
}
