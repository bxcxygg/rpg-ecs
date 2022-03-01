use bevy::prelude::{App, Schedule, Stage, World};
use gdnative::prelude::*;

use crate::{
    app::get_ecs,
    engine_sync::{
        events::{spawn_game, spawn_node, update_delta_resource, user_input},
        resources::{IdleDelta, PhysicsDelta},
    },
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
        spawn_node(&mut self.world, other, name);
    }

    #[export]
    fn add_game_to_ecs(&mut self, _owner: &Node, other: Ref<Node>) {
        spawn_game(&mut self.world, other);
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
