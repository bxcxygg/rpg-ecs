use bevy::app::PluginGroupBuilder;
use bevy::prelude::{App, Plugin, PluginGroup};

use crate::player::player::add_player_system;

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_player_system);
    }
}

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(GamePlugin);
    }
}
