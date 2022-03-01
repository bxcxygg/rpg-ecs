use bevy::{diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*};

use crate::engine_sync::{EngineSyncPlugin, GamePlugin};

pub fn get_ecs() -> App {
    let mut ecs = App::new();
    ecs.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(EngineSyncPlugin)
        .add_plugin(GamePlugin); // order matters here

    ecs
}
