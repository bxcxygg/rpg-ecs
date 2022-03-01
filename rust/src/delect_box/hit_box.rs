use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::Ref;

#[derive(Component)]
pub struct HitBox {
    pub(crate) node: Ref<Area2D>,
    pub(crate) damage: i32,
}
