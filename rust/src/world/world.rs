use bevy::prelude::{DespawnRecursiveExt, Entity};
use gdnative::api::MainLoop;
use gdnative::prelude::*;
use gdrust::ecs::app::with_world;
use gdrust::ecs::engine_sync::components::{GameNode, GodotObjRef, PlayingGame};
use gdrust::ecs::engine_sync::events::spawn_game;
use gdrust::macros::*;

#[gdrust(extends = Node2D)]
#[derive(Default, Clone)]
pub struct WorldBundle {
    pub entity: Option<Entity>,
}

#[methods]
impl WorldBundle {
    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        with_world(|w| {
            let entity = w
                .spawn()
                .insert(GodotObjRef::new(owner.claim()))
                .insert(GameNode)
                .insert(PlayingGame)
                .id();
            self.entity = Some(entity);
            spawn_game(w, entity);
        });
    }

    #[export]
    fn _notification(&mut self, _owner: TRef<Node2D>, what: i64) {
        if what == 1 || what == MainLoop::NOTIFICATION_WM_GO_BACK_REQUEST {
            with_world(|w| {
                w.entity_mut(self.entity.unwrap()).despawn_recursive();
            });
        }
    }
}
