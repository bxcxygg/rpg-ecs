use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Timer};
use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;
use gdrust::unsafe_functions::{PackedSceneExt, RefExt};

#[derive(Component)]
pub struct Effect {
    pub node: Ref<AnimatedSprite>,
}

impl Effect {
    pub fn new(pos: Vector2, effect: Ref<PackedScene>, parent: TRef<Node>) -> Self {
        let effect = effect.expect_safe();
        let effect = effect.expect_instance_as::<AnimatedSprite>();

        effect.set_global_position(pos);
        parent.add_child(effect, false);

        effect.set_frame(0);
        effect.play("animate", false);

        Self {
            node: effect.claim(),
        }
    }
}

pub fn add_effect(
    mut commands: EntityCommands,
    effect: Ref<PackedScene>,
    time: f32,
    pos: Vector2,
    parent: TRef<Node>,
) {
    commands
        .insert(Effect::new(pos, effect, parent))
        .insert(Timer::from_seconds(time, false));
}

pub fn effect_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut effect: Query<(Entity, &Effect, &mut Timer)>,
) {
    for (entity, effect, mut timer) in effect.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let effect = effect.node.expect_safe();

            effect.queue_free();
            commands.entity(entity).despawn();
        }
    }
}
