use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::macros::*;

use crate::effect::HitEffect;

/// HurtBox Component.
#[gdrust(extends = Area2D)]
#[derive(Component, Clone)]
pub struct HurtBox {
    #[default(_owner.claim())]
    pub owner: Ref<Area2D>,
    pub hit_effect: HitEffect,
}

#[methods]
impl HurtBox {}
