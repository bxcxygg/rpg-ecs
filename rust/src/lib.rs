use gdnative::prelude::{godot_init, InitHandle};

mod app;
mod components;
mod delect_box;
mod ecs_controller;
mod engine_sync;
mod player;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<ecs_controller::ECSController>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
