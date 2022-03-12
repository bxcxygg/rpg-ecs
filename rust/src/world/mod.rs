use crate::world::grass::kill_grass_system;
use crate::world::health::{
    set_health_system, set_max_health_system, ChangeHealth, ChangeMaxHealth,
};
use bevy::prelude::{App, Plugin};

pub mod grass;
pub mod health;
pub mod world;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeHealth>()
            .add_event::<ChangeMaxHealth>()
            .add_system(kill_grass_system)
            .add_system(set_health_system)
            .add_system(set_max_health_system);
    }
}
