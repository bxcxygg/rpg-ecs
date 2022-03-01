use bevy::prelude::Component;
use gdnative::api::{AnimatedSprite, AnimationNodeStateMachinePlayback, AnimationTree};
use gdnative::prelude::*;
use gdrust::macros::single_value;

#[derive(Component, Debug)]
#[single_value(extends = i32)]
pub struct Damage {
    pub value: i32,
}

#[derive(Component)]
#[single_value(extends = Ref < AnimatedSprite >)]
pub struct Effect {
    pub value: Ref<AnimatedSprite>,
}

#[derive(Component, Debug)]
pub struct Stats {
    pub max_health: i32,
    pub health: i32,
}

#[derive(Component, Debug)]
pub struct Roll {
    pub roll_speed: f32,
    pub roll_velocity: Vector2,
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub max_speed: f32,
    pub acceleration: f32,
}

#[derive(Component)]
#[single_value(extends = f32)]
pub struct Friction {
    pub value: f32,
}

#[derive(Component)]
pub struct AnimationTreeComponent {
    pub animation_tree: Ref<AnimationTree>,
    pub animation_state: Ref<AnimationNodeStateMachinePlayback>,
}

#[derive(Component)]
#[single_value(extends = Vector2)]
pub struct Velocity {
    pub value: Vector2,
}
