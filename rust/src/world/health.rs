use bevy::prelude::{Component, EventReader, Query, With};
use gdnative::api::TextureRect;
use gdnative::prelude::*;
use gdrust::ecs::app::with_world;
use gdrust::ecs::engine_sync::components::PlayingGame;
use gdrust::macros::*;
use gdrust::unsafe_functions::{NodeExt, RefExt};

pub struct ChangeHealth {
    pub health: i32,
}
pub struct ChangeMaxHealth {
    pub max_health: i32,
}

#[derive(Component, Copy, Clone)]
pub struct HeartEmpty(pub Ref<TextureRect>);

#[derive(Component, Copy, Clone)]
pub struct HeartFull(pub Ref<TextureRect>);

#[derive(Component, Copy, Clone)]
pub struct Health;

#[gdrust(extends = Control)]
#[derive(Default, Clone)]
pub struct HealthBundle;
#[methods]
impl HealthBundle {
    #[export]
    fn _ready(&mut self, owner: TRef<Control>) {
        with_world(|w| {
            let heart_empty = owner.expect_node::<TextureRect>("HeartUIEmpty").claim();
            let heart_full = owner.expect_node::<TextureRect>("HeartUIFull").claim();

            w.spawn()
                .insert(HeartEmpty(heart_empty))
                .insert(HeartFull(heart_full))
                .insert(Health)
                .insert(PlayingGame);
        });
    }
}

pub fn set_health_system(
    health: Query<&HeartFull, With<Health>>,
    mut evnet: EventReader<ChangeHealth>,
) {
    for heart_full in health.iter() {
        let heart_full = heart_full.0.expect_safe();

        for ChangeHealth { health } in evnet.iter() {
            heart_full.set_size(
                Vector2::new(*health as f32 * 15.0, heart_full.size().y),
                false,
            );
        }
    }
}

pub fn set_max_health_system(
    health: Query<&HeartEmpty, With<Health>>,
    mut evnet: EventReader<ChangeMaxHealth>,
) {
    for heart_empty in health.iter() {
        let heart_empty = heart_empty.0.expect_safe();

        for ChangeMaxHealth { max_health } in evnet.iter() {
            heart_empty.set_size(
                Vector2::new(*max_health as f32 * 15.0, heart_empty.size().y),
                false,
            );
        }
    }
}
