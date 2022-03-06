mod app;
mod components;
mod delect_box;
mod effect;
mod player;
mod world;

use crate::app::get_ecs;
use crate::effect::grass_effect::EffectFinished;
use crate::player::SpawnPlayer;
use crate::world::{AreaIntoGrass, SpawnGrass};
use bevy::prelude::{App, Schedule, Stage, World};
use gdnative::api::{AnimatedSprite, Area2D};
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::{
    events::{spawn_game, spawn_node, update_delta_resource, user_input},
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
    world: World,
    schedule: Schedule,
}

#[methods]
impl ECSController {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &Node) -> Self {
        godot_print!("ECSController is created!");
        let App {
            world, schedule, ..
        } = get_ecs();
        ECSController {
            name: "".to_string(),
            world,
            schedule,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        self.name = "ECSController".to_string();
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f32) {
        self.world.clear_trackers();
        update_delta_resource::<IdleDelta>(&mut self.world, delta);
        self.schedule.run(&mut self.world);
    }

    /// I created two Detlta resources, one for the physics loop, and one for the Idle loop
    #[export]
    fn _physics_process(&mut self, _owner: &Node, delta: f32) {
        update_delta_resource::<PhysicsDelta>(&mut self.world, delta);
    }

    #[export]
    fn add_node_to_ecs(&mut self, _owner: &Node, other: Ref<Node>, name: String) {
        match name.as_str() {
            "Player" => spawn_node(&mut self.world, SpawnPlayer { node: other }),
            "Grass" => spawn_node(&mut self.world, SpawnGrass { node: other }),
            _ => (),
        }
    }

    #[export]
    fn add_game_to_ecs(&mut self, _owner: &Node, other: Ref<Node>) {
        spawn_game(&mut self.world, other);
    }

    #[export]
    fn add_signal_to_ecs(&mut self, _owner: &Node, name: String, vars: VariantArray) {
        match name.as_str() {
            "grass/_on_area_entered" => spawn_node(
                &mut self.world,
                AreaIntoGrass {
                    grass: vars.get(0).try_to::<Ref<Node2D>>().unwrap(),
                    area: vars.get(1).try_to::<Ref<Area2D>>().unwrap(),
                },
            ),
            "Effect/_on_animation_finished" => spawn_node(
                &mut self.world,
                EffectFinished {
                    effect: vars.get(0).try_to::<Ref<AnimatedSprite>>().unwrap(),
                },
            ),
            _ => (),
        }
    }

    #[export]
    fn _input(&mut self, _owner: &Node, event: Ref<InputEvent>) {
        let event = unsafe { event.assume_safe() };
        if !event.is_action_type() {
            return;
        }
        user_input(&mut self.world, event);
    }
}
fn init(handle: InitHandle) {
    handle.add_class::<ECSController>();
}
godot_init!(init);
