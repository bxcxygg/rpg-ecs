use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::macros::*;
use gdrust::unsafe_functions::ResourceLoaderExt;

pub const HIT_EFFECT_LENGTH: f32 = 2. / 15.;

/// HurtBox Component.
#[gdrust(extends = Area2D)]
#[derive(Component, Clone)]
pub struct HurtBox {
    #[default(_owner.claim())]
    pub owner: Ref<Area2D>,
    #[default(ResourceLoader::godot_singleton().expect_load_scene("res://scenes/effect/HitEffect.tscn"))]
    pub effect: Ref<PackedScene>,
}
#[methods]
impl HurtBox {}
