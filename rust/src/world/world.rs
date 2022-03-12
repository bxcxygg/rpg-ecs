use crate::with_world;
use gdnative::prelude::*;
use gdrust::ecs::engine_sync::events::spawn_game;
use gdrust::macros::*;

#[gdrust(extends = Node2D)]
#[derive(Default, Clone)]
pub struct WorldBundle;
#[methods]
impl WorldBundle {
    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        with_world(|w| spawn_game(w, owner.upcast::<Node>().claim()))
    }
}
