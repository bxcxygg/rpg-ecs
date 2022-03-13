#![feature(derive_default_enum)]

mod app;
mod components;
mod delect_box;
mod effect;
mod enemy;
mod player;
mod world;

use crate::app::{init_ecs, with_schedule, with_world};
use crate::delect_box::hit_box::HitBox;
use crate::delect_box::hurt_box::HurtBox;
use crate::delect_box::soft_collision::SoftCollision;
use crate::effect::EffectPlugin;
use crate::enemy::bat::BatBundle;
use crate::enemy::wander_controller::WanderController;
use crate::enemy::EnemyPlugin;
use crate::player::{PlayerBundle, PlayerPlugin};
use crate::world::grass::Grass;
use crate::world::health::HealthBundle;
use crate::world::world::WorldBundle;
use crate::world::WorldPlugin;
use bevy::app::App;
use bevy::prelude::{Plugin, Stage};
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::{
    events::update_delta_resource,
    resources::{IdleDelta, PhysicsDelta},
};

/// This ECSController acts as the middle man between Godot and Bevy, it's a singleton or "AutoLoad" script that
/// creates the entire Bevy ECS. Also, "Project Settings > Rendering > Threading" to turn on multi threading, which will work
/// nicely with the multi threading that Bevy offers, ie, if you want to render multiple things using ecs, then Godot will play nicely.
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct ECSController {
    name: String,
}

#[methods]
impl ECSController {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &Node) -> Self {
        godot_print!("ECSController is created!");
        ECSController {
            name: "".to_string(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        self.name = "ECSController".to_string();
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f32) {
        with_world(|w| {
            w.clear_trackers();
            update_delta_resource::<IdleDelta>(w, delta);
            with_schedule(|s| s.run(w));
        });
    }

    /// I created two Detlta resources, one for the physics loop, and one for the Idle loop
    #[export]
    fn _physics_process(&mut self, _owner: &Node, delta: f32) {
        with_world(|w| {
            update_delta_resource::<PhysicsDelta>(w, delta);
        });
    }
}

struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(EffectPlugin);
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

    init_ecs(GamePlugin);
}

godot_init!(init);
