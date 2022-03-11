use crate::effect::Effect;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Entity, Query, Res, Time, Timer};
use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;
use gdrust::unsafe_functions::{PackedSceneExt, RefExt};

pub fn add_effect(
    mut commands: EntityCommands,
    effect: Ref<PackedScene>,
    time: f32,
    pos: Vector2,
    parent: TRef<Node>,
) {
    let effect = Effect::new(effect);
    effect.play(pos, parent);

    commands
        .insert(effect)
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
            let effect = effect.effect.expect_safe();
            let effect = effect.expect_instance_as::<AnimatedSprite>();

            effect.queue_free();
            commands.entity(entity).despawn();
        }
    }
}
