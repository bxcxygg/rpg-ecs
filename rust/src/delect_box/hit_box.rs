use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::macros::*;

/// HitBox Component.
#[gdrust(extends = Area2D)]
#[derive(Component, Copy, Clone)]
pub struct HitBox {
    #[default(_owner.claim())]
    pub owner: Ref<Area2D>,
    #[export]
    #[default(1)]
    pub damage: i32,
    pub knockback: Vector2,
}
#[methods]
impl HitBox {}
