use crate::{
    game::Game,
    moves::player_move_or_attack,
    object::{Object, PLAYER},
    renderer::Tcod,
};
use tcod::input::Key;
use tcod::input::KeyCode::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> PlayerAction {
    use PlayerAction::*;

    let player_alive = objects[PLAYER].alive;

    match (tcod.key, tcod.key.text(), player_alive) {
        (
            Key {
                code: Enter,
                alt: true,
                ..
            },
            _,
            _,
        ) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
        (Key { code: Escape, .. }, _, _) => Exit,
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(PLAYER, 0, -1, objects, game);
            TookTurn
        }
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(PLAYER, 0, 1, objects, game);
            TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(PLAYER, -1, 0, objects, game);
            TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(PLAYER, 1, 0, objects, game);
            TookTurn
        }
        _ => DidntTakeTurn,
    }
}
