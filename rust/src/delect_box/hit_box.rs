use bevy::prelude::Component;
use gdnative::api::{Area2D, Position2D};
use gdnative::prelude::{Ref, Vector2};
use gdrust::macros::gdcomponent;
use gdrust::unsafe_functions::{NodeExt, RefExt};

use crate::components::Damage;

#[gdcomponent(extends = Area2D)]
pub struct HitBox {
    #[node]
    pub(crate) node: Ref<Area2D>,
}

#[gdcomponent(extends = Position2D)]
pub struct HitBoxPosition {
    #[node]
    pub(crate) node: Ref<Position2D>,
    #[component("SwordHitbox")]
    pub(crate) hit_box: HitBox,
    #[component("Damage")]
    pub(crate) damage: Damage,
    pub knockback_vector: Vector2,
}
