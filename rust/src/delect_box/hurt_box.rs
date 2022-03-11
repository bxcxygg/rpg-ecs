use crate::effect::Effect;
use bevy::prelude::{Bundle, Component};
use defaults::Defaults;
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::unsafe_functions::ResourceLoaderExt;

pub const HIT_EFFECT_LENGTH: f32 = 2. / 15.;

/// HurtBox Component.
#[derive(Component, Defaults, Copy, Clone)]
pub struct HurtBox {
    #[def = "Area2D::new().into_shared()"]
    pub owner: Ref<Area2D>,
}

#[derive(Bundle, NativeClass, Default, Clone)]
#[inherit(Area2D)]
pub struct HurtBoxBundle {
    pub hurtbox: HurtBox,
    pub effect: Effect,
}

#[methods]
impl HurtBoxBundle {
    fn new(owner: TRef<Area2D>) -> Self {
        Self {
            hurtbox: HurtBox {
                owner: owner.claim(),
            },
            effect: Effect::new(
                ResourceLoader::godot_singleton()
                    .expect_load_scene("res://scenes/effect/HitEffect.tscn"),
            ),
        }
    }
}
