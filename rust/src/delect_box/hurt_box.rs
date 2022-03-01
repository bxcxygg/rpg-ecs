use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::*;

#[derive(Component)]
pub struct HurtBox {
    pub(crate) node: Ref<Area2D>,
}
