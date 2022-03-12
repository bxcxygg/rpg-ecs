use bevy::ecs::system::EntityCommands;
use bevy::prelude::{App, Commands, Component, Entity, Plugin, Query, Res, Time, Timer};
use defaults::Defaults;
use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::GodotObjRef;
use gdrust::unsafe_functions::{PackedSceneExt, RefExt};

#[derive(Component, Default, Clone)]
pub struct EffectTimer(pub Timer);

#[derive(Component, Defaults, Clone)]
pub struct Effect {
    #[def = "PackedScene::new().into_shared()"]
    pub effect: Ref<PackedScene>,
}
impl Effect {
    pub fn new(effect: Ref<PackedScene>) -> Self {
        Self { effect }
    }
}

pub fn add_effect(
    mut commands: EntityCommands,
    effect: Ref<PackedScene>,
    time: f32,
    pos: Vector2,
    parent: TRef<Node>,
) {
    let effect = effect.expect_safe();
    let effect = effect.expect_instance_as::<AnimatedSprite>();

    effect.set_global_position(pos);
    parent.add_child(effect, false);

    effect.set_frame(0);
    effect.play("animate", false);

    commands
        .insert(GodotObjRef::new(effect.claim()))
        .insert(EffectTimer(Timer::from_seconds(time, false)));
}

pub fn effect_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut effect: Query<(Entity, &GodotObjRef<AnimatedSprite>, &mut EffectTimer)>,
) {
    for (entity, effect, mut timer) in effect.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            effect.expect_safe().queue_free();
            commands.entity(entity).despawn();
        }
    }
}

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(effect_finished);
    }
}
