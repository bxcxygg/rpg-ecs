pub use crate::world::grass::*;
use bevy::prelude::{App, Plugin};

mod grass;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnGrass>()
            .add_system(add_grass_system)
            .add_system(kill_grass_system);
    }
}
