use bevy::prelude::{App, Commands, Component, Entity, Plugin, Query, Res, Time, Timer};
use defaults::Defaults;
use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::components::GodotObjRef;
use gdrust::macros::gdrust;
use gdrust::unsafe_functions::{PackedSceneExt, RefExt, ResourceLoaderExt};

#[derive(Component, Default, Clone)]
pub struct EffectTimer(pub Timer);

#[derive(Defaults, Clone)]
pub struct BatDeadEffect {
    #[def = "ResourceLoader::godot_singleton().expect_load_scene(\"res://scenes/effect/EnemyDeathEffect.tscn\")"]
    pub effect: Ref<PackedScene>,
}

#[derive(Defaults, Clone)]
pub struct HitEffect {
    #[def = "ResourceLoader::godot_singleton().expect_load_scene(\"res://scenes/effect/HitEffect.tscn\")"]
    pub effect: Ref<PackedScene>,
}

#[derive(Defaults, Clone)]
pub struct GrassEffect {
    #[def = "ResourceLoader::godot_singleton().expect_load_scene(\"res://scenes/effect/GrassEffect.tscn\")"]
    pub effect: Ref<PackedScene>,
}

#[gdrust(extends = AnimatedSprite)]
#[derive(Component, Clone)]
pub struct Effect;

#[methods]
impl Effect {
    #[export]
    fn _ready(&mut self, owner: TRef<AnimatedSprite>) {
        owner.set_frame(0);
        owner.play("animate", false);
    }
}

pub fn add_effect(
    commands: &mut Commands,
    effect: &Ref<PackedScene>,
    pos: Vector2,
    parent: TRef<Node>,
) {
    let effect = effect.expect_safe();
    let effect = effect.expect_instance_as::<AnimatedSprite>();
    let frame = effect
        .sprite_frames()
        .unwrap()
        .expect_safe()
        .get_frame_count("animate");

    effect.set_global_position(pos);
    parent.add_child(effect, false);

    commands
        .spawn()
        .insert(GodotObjRef::new(effect.claim()))
        .insert(EffectTimer(Timer::from_seconds(frame as f32 / 15., false)));
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
