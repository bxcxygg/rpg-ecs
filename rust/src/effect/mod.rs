use crate::effect::grass_effect::effect_finished;
use bevy::prelude::{App, Component, Plugin};
use defaults::Defaults;
use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;
use gdrust::unsafe_functions::{PackedSceneExt, RefExt};

pub mod grass_effect;

#[derive(Component, Defaults, Clone)]
pub struct Effect {
    #[def = "PackedScene::new().into_shared()"]
    pub effect: Ref<PackedScene>,
}

impl Effect {
    pub fn new(effect: Ref<PackedScene>) -> Self {
        Self { effect }
    }

    pub fn play(&self, pos: Vector2, parent: TRef<Node>) {
        let effect = self.effect.expect_safe();
        let effect = effect.expect_instance_as::<AnimatedSprite>();

        effect.set_global_position(pos);
        parent.add_child(effect, false);

        effect.set_frame(0);
        effect.play("animate", false);
    }
}

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(effect_finished);
    }
}
