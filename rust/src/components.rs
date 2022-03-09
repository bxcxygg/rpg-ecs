use bevy::prelude::Component;
use gdnative::prelude::*;
use std::ops::{Deref, DerefMut};

/// Stats Component.
#[derive(Component, NativeClass, Default)]
#[inherit(Node)]
#[user_data(user_data::RwLockData<Stats>)]
pub struct Stats {
    #[property(default = 4)]
    pub max_health: i32,
    #[property(default = 4, get, set = "Self::set_health")]
    pub health: i32,
}
#[methods]
impl Stats {
    fn new(_: &Node) -> Self {
        Self {
            max_health: 4,
            health: 4,
        }
    }

    pub fn set_health(this: &mut Stats, _: TRef<Node>, health: i32) {
        this.health = if health > this.max_health {
            this.max_health
        } else {
            health
        }
    }
}

/// Roll Component.
#[derive(Component, NativeClass, Default)]
#[inherit(Node)]
#[user_data(user_data::RwLockData<Roll>)]
pub struct Roll {
    #[property(default = 120.0)]
    pub roll_speed: f32,
    pub roll_velocity: Vector2,
}
#[methods]
impl Roll {
    fn new(_: &Node) -> Self {
        Self {
            roll_speed: 120.0,
            ..Default::default()
        }
    }
}

/// Accelerates Component.
#[derive(Component, NativeClass, Default)]
#[inherit(Node)]
#[user_data(user_data::RwLockData<Acceleration>)]
pub struct Acceleration {
    #[property(default = 80.0)]
    pub max_speed: f32,
    #[property(default = 500.0)]
    pub acceleration_speed: f32,
}
#[methods]
impl Acceleration {
    fn new(_: &Node) -> Self {
        Self {
            max_speed: 80.0,
            acceleration_speed: 500.0,
        }
    }
}

/// Friction Component.
#[derive(Component, NativeClass, Default)]
#[inherit(Node)]
#[user_data(user_data::RwLockData<Friction>)]
pub struct Friction {
    #[property(default = 400.0)]
    pub friction: f32,
}
#[methods]
impl Friction {
    fn new(_: &Node) -> Self {
        Self { friction: 400.0 }
    }
}

/// Velocity Component.
#[derive(Component, Default)]
pub struct Velocity {
    pub velocity: Vector2,
}

impl Deref for Velocity {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.velocity
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.velocity
    }
}
