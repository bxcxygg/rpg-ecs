use bevy::prelude::{Component, Query, Res, Time, Timer};
use gdnative::prelude::*;
use gdrust::macros::*;
use rand::{thread_rng, Rng};
use std::ops::Range;

#[derive(Component, Clone)]
pub struct WanderTimer(pub Timer);

/// WanderController Component.
#[gdrust(extends = Node2D)]
#[derive(Component, Copy, Clone)]
pub struct WanderController {
    #[default(_owner.claim())]
    pub owner: Ref<Node2D>,
    #[export]
    #[default(32.)]
    wander_range: f32,
    pub start_position: Vector2,
    pub target_position: Vector2,
}
#[methods]
impl WanderController {
    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        self.start_position = owner.global_position();
        self.target_position = owner.global_position();
    }
}

pub fn update_target_position_system(
    time: Res<Time>,
    mut wander_controller: Query<(&mut WanderController, &mut WanderTimer)>,
) {
    let mut rng = thread_rng();

    for (mut wander_controller, mut timer) in wander_controller.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let target_vector = Vector2::new(
                rng.gen_range(Range {
                    start: -wander_controller.wander_range,
                    end: wander_controller.wander_range,
                }),
                rng.gen_range(Range {
                    start: -wander_controller.wander_range,
                    end: wander_controller.wander_range,
                }),
            );

            wander_controller.target_position = wander_controller.start_position + target_vector;
        }
    }
}
