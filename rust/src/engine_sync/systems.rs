use bevy::prelude::*;
use gdnative::api::*;

use super::{
    components::{GameNode, GodotObjRef, PlayingGame},
    events::{DespawnPlayingGame, SpawnGame},
    resources::GameOver,
};

pub fn spawn_game(
    mut commands: Commands,
    mut on_spawn_game: EventReader<SpawnGame>,
    children: Query<Entity, With<PlayingGame>>,
) {
    for SpawnGame { node } in on_spawn_game.iter() {
        let mut parent = commands.spawn();
        parent
            .insert(GodotObjRef::new(node.clone()))
            .insert(GameNode)
            .insert(PlayingGame);
        for child in children.iter() {
            parent.add_child(child);
        }
    }
}

pub fn end_game(
    game_over: Res<Option<GameOver>>,
    mut send_despawn_playing_game: EventWriter<DespawnPlayingGame>,
    game: Query<&GodotObjRef<Node>, With<GameNode>>,
) {
    if game_over.is_changed() && game_over.is_some() {
        for game_node in game.iter() {
            let game_node = unsafe { game_node.assume_safe() };

            game_node.queue_free();
            send_despawn_playing_game.send(DespawnPlayingGame);
        }
    }
}

pub fn despawn_playing_game(
    mut commands: Commands,
    mut on_despawn_playing_game: EventReader<DespawnPlayingGame>,
    playing_game: Query<Entity, With<PlayingGame>>,
) {
    for _despawn_playing_game in on_despawn_playing_game.iter() {
        for entity in playing_game.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
