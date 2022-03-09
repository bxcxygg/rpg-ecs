use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::unsafe_functions::ResourceLoaderExt;

pub const HIT_EFFECT_LENGTH: f32 = 2. / 15.;

#[derive(Component, NativeClass)]
#[inherit(Area2D)]
#[user_data(user_data::RwLockData<HurtBox>)]
pub struct HurtBox {
    pub effect: Ref<PackedScene>,
}
#[methods]
impl HurtBox {
    fn new(_: &Area2D) -> Self {
        Self {
            effect: ResourceLoader::godot_singleton()
                .expect_load_scene("res://scenes/effect/HitEffect.tscn"),
        }
    }
}
