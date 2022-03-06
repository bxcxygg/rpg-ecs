use bevy::prelude::Component;
use gdnative::api::{
    AnimatedSprite, AnimationNodeStateMachinePlayback, AnimationPlayer, AnimationTree,
};
use gdnative::prelude::*;
use gdrust::macros::*;
use gdrust::unsafe_functions::{NodeExt, RefExt};

#[gdcomponent(extends = Node)]
pub struct Damage {
    #[node]
    pub node: Ref<Node>,
    #[property("damage")]
    pub damage: i32,
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

#[derive(Component)]
pub struct Animation {
    pub animation_player: Ref<AnimationPlayer>,
    pub animation_tree: Ref<AnimationTree>,
    pub animation_state: Ref<AnimationNodeStateMachinePlayback>,
}

impl Animation {
    pub fn new(node: TRef<Node>) -> Self {
        let animation_tree = node.expect_node::<AnimationTree, &str>("AnimationTree");
        let animation_player = node.expect_node::<AnimationPlayer, &str>("AnimationPlayer");
        let animation_state = animation_tree
            .get("parameters/playback")
            .try_to_object::<AnimationNodeStateMachinePlayback>()
            .expect("Could not get AnimationNodeStateMachinePlayback");

        animation_tree.set_active(true);

        Self {
            animation_player: animation_player.claim(),
            animation_tree: animation_tree.claim(),
            animation_state,
        }
    }
}

#[single_value(extends = Vector2)]
#[derive(Component)]
pub struct Velocity {
    pub value: Vector2,
}
