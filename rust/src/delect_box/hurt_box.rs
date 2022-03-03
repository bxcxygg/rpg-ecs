use bevy::prelude::Component;
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::gdrust_macros::gdcomponent;
use gdrust::unsafe_functions::RefExt;

#[gdcomponent(extends = Area2D)]
pub struct HurtBox {
    #[node]
    pub(crate) node: Ref<Area2D>,
}
