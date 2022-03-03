use bevy::prelude::Component;
use gdnative::api::{AnimatedSprite, AnimationNodeStateMachinePlayback, AnimationTree};
use gdnative::prelude::*;
use gdrust::gdrust_macros::{gdcomponent, single_value};
use gdrust::unsafe_functions::{NodeExt, RefExt};

#[gdcomponent(extends = Node)]
pub struct Damage {
    #[node]
    pub node: Ref<Node>,
    #[property("damage")]
    pub damage: i32,
}

#[gdcomponent(extends = AnimatedSprite)]
pub struct Effect {
    #[node]
    pub node: Ref<AnimatedSprite>,
}

#[gdcomponent(extends = Node)]
pub struct Stats {
    #[node]
    pub node: Ref<Node>,
    #[property("max_health")]
    pub max_health: i32,
    #[property("health")]
    pub health: i32,
}

#[gdcomponent(extends = Node)]
pub struct Roll {
    #[node]
    pub node: Ref<Node>,
    #[property("roll_speed")]
    pub roll_speed: f32,
    pub roll_velocity: Vector2,
}

#[gdcomponent(extends = Node)]
pub struct Acceleration {
    #[node]
    pub node: Ref<Node>,
    #[property("max_speed")]
    pub max_speed: f32,
    #[property("acceleration")]
    pub acceleration: f32,
}

#[gdcomponent(extends = Node)]
pub struct Friction {
    #[node]
    pub node: Ref<Node>,
    #[property("friction")]
    pub value: f32,
}

#[gdcomponent(extends = AnimationTree)]
pub struct AnimationTreeComponent {
    #[node]
    pub animation_tree: Ref<AnimationTree>,
    #[value(AnimationTreeComponent::set_animation_state(node))]
    pub animation_state: Ref<AnimationNodeStateMachinePlayback>,
}

impl AnimationTreeComponent {
    pub fn set_animation_state(node: TRef<Node>) -> Ref<AnimationNodeStateMachinePlayback> {
        let animation_tree = node.cast::<AnimationTree>().unwrap();
        animation_tree
            .get("parameters/playback")
            .try_to_object::<AnimationNodeStateMachinePlayback>()
            .expect("Could not get AnimationNodeStateMachinePlayback")
    }
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vector2,
}
