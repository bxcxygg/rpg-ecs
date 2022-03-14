#![feature(derive_default_enum)]

mod components;
mod delect_box;
mod effect;
mod enemy;
mod player;
mod world;

use crate::delect_box::hit_box::HitBox;
use crate::delect_box::hurt_box::HurtBox;
use crate::delect_box::soft_collision::SoftCollision;
use crate::effect::{Effect, EffectPlugin};
use crate::enemy::bat::BatBundle;
use crate::enemy::wander_controller::WanderController;
use crate::enemy::EnemyPlugin;
use crate::player::{PlayerBundle, PlayerPlugin};
use crate::world::grass::Grass;
use crate::world::health::HealthBundle;
use crate::world::world::WorldBundle;
use crate::world::WorldPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;
use gdnative::prelude::*;
use gdrust::ecs::app::init_ecs;
use gdrust::ecs::engine_controller::ECSController;

struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin)
            .add_plugin(EffectPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<ECSController>();
    handle.add_class::<HitBox>();
    handle.add_class::<HurtBox>();
    handle.add_class::<PlayerBundle>();
    handle.add_class::<BatBundle>();
    handle.add_class::<Grass>();
    handle.add_class::<WorldBundle>();
    handle.add_class::<HealthBundle>();
    handle.add_class::<SoftCollision>();
    handle.add_class::<WanderController>();
    handle.add_class::<Effect>();

    init_ecs(GamePlugin);
}

godot_init!(init);
