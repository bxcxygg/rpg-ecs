use bevy::app::EventReader;
use bevy::prelude::{Commands, Component, Entity, Query};
use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;
use gdrust::unsafe_functions::{PackedSceneExt, RefExt};

pub struct EffectFinished {
    pub effect: Ref<AnimatedSprite>,
}

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

        effect
            .connect(
                "animation_finished",
                effect,
                "_on_animation_finished",
                VariantArray::new_shared(),
                0,
            )
            .expect("Failed to connect to animation_finished signal");
        effect.set_frame(0);
        effect.play("animate", false);

        Self {
            node: effect.claim(),
        }
    }
}

pub fn effect_finished(
    mut commands: Commands,
    mut finish_effect: EventReader<EffectFinished>,
    effect: Query<(Entity, &Effect)>,
) {
    for EffectFinished {
        effect: finish_effect,
    } in finish_effect.iter()
    {
        for (entity, effect) in effect.iter() {
            let finish_effect = finish_effect.expect_safe();
            let effect = effect.node.expect_safe();

            if finish_effect.get_instance_id() == effect.get_instance_id() {
                effect.queue_free();
                commands.entity(entity).despawn();
            }
        }
    }
}
