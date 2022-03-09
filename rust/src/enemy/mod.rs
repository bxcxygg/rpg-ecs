use crate::enemy::bat::{
    add_bat_system, attack_bat_system, bat_chase_system, bat_idle_system, bat_move_system,
};
use crate::player::Player;
use crate::SpawnBat;
use bevy::app::Plugin;
use bevy::prelude::{Component, Query};
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::stages::SyncStages;
use gdrust::macros::gdcomponent;
use gdrust::unsafe_functions::RefExt;

pub mod bat;

#[derive(Component)]
pub struct Knockback {
    pub knockback: Vector2,
}

#[gdcomponent(extends = Area2D)]
pub struct DelectionZone {
    #[node]
    pub node: Ref<Area2D>,
    pub player: Option<Ref<KinematicBody2D>>,
}

fn zone_system(mut zone: Query<&mut DelectionZone>, player: Query<&Player>) {
    for mut zone in zone.iter_mut() {
        let zone_area = zone.node.expect_safe();

        for player in player.iter() {
            let player = player.node.expect_safe();
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
        app.add_event::<SpawnBat>()
            .add_system(zone_system)
            .add_system(add_bat_system)
            .add_system(bat_idle_system)
            .add_system(bat_chase_system)
            .add_system(attack_bat_system)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, bat_move_system);
    }
}
