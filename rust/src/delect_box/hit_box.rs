use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::*;

#[derive(Component, NativeClass, Default)]
#[inherit(Area2D)]
#[user_data(user_data::RwLockData<HitBox>)]
pub struct HitBox {
    #[property(default = 1)]
    pub damage: i32,
    pub knockback_vector: Vector2,
}
#[methods]
impl HitBox {
    fn new(_: &Area2D) -> Self {
        Default::default()
    }
}
