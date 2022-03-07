use crate::effect::grass_effect::effect_finished;
use bevy::prelude::{App, Plugin};

pub mod grass_effect;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(effect_finished);
    }
}
