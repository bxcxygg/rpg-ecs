use bevy::prelude::{Component, Query};
use gdnative::api::Area2D;
use gdnative::prelude::*;
use gdrust::macros::*;
use gdrust::unsafe_functions::RefExt;

/// HurtBox Component.
#[gdrust(extends = Area2D)]
#[derive(Component, Clone)]
pub struct SoftCollision {
    #[default(_owner.claim())]
    pub owner: Ref<Area2D>,
    pub input_vector: Vector2,
}
#[methods]
impl SoftCollision {}

pub fn soft_collision_system(mut soft_collision: Query<&mut SoftCollision>) {
    for mut soft_collision in soft_collision.iter_mut() {
        let soft_collision_area = soft_collision.owner.expect_safe();
        let areas = soft_collision_area.get_overlapping_areas();

        if !areas.is_empty() {
            let area = areas
                .get(0)
                .try_to_object::<Area2D>()
                .unwrap()
                .expect_safe();
            soft_collision.input_vector =
                area.global_position() - soft_collision_area.global_position();
        } else {
            soft_collision.input_vector = Vector2::ZERO;
        }
    }
}
