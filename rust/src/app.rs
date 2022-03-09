use std::sync::Mutex;
// use crate::effect::EffectPlugin;
// use crate::enemy::EnemyPlugin;
// use crate::world::WorldPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::{App, Plugin, Schedule, World};
use bevy::MinimalPlugins;
use gdrust::ecs::engine_sync::EngineSyncPlugin;

lazy_static::lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::default());
    static ref SCHEDULE: Mutex<Schedule> = Mutex::new(Schedule::default());
}

pub fn with_world<F>(mut f: F)
where
    F: FnMut(&mut World),
{
    let _result = WORLD.try_lock().map(|mut world| f(&mut world));
}

pub fn with_schedule<F>(mut f: F)
where
    F: FnMut(&mut Schedule),
{
    let _result = SCHEDULE.try_lock().map(|mut schedule| f(&mut schedule));
}

pub fn init_ecs<T: Plugin>(game_plugin: T)
where
    T: Plugin,
{
    let get_ecs = move || {
        let mut ecs = App::new();

        ecs.add_plugins(MinimalPlugins)
            .add_plugin(LogPlugin)
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(EngineSyncPlugin)
            .add_plugin(game_plugin);
        // .add_plugin(WorldPlugin)
        // .add_plugin(EffectPlugin)
        // .add_plugin(EnemyPlugin);

        ecs
    };

    let App {
        schedule, world, ..
    } = get_ecs();

    *WORLD.lock().unwrap() = world;
    *SCHEDULE.lock().unwrap() = schedule;
}
