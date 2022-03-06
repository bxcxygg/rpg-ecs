use crate::effect::EffectPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::App;
use bevy::MinimalPlugins;
use gdrust::ecs::engine_sync::EngineSyncPlugin;

pub fn get_ecs() -> App {
    let mut ecs = App::new();
    ecs.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(EngineSyncPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(EffectPlugin);

    ecs
}
