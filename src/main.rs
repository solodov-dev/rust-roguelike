use rust_roguelike::game::*;
use rust_roguelike::input::*;
use rust_roguelike::map::*;
use rust_roguelike::object::*;
use rust_roguelike::renderer::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/litcod tutorial")
        .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
    };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Object::new(0, 0, '@', WHITE, "player", true);
    player.alive = true;

    let mut objects = vec![player];

    let mut game = Game::new(&mut objects);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            );
        }
    }

    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        tcod.con.clear();

        for object in &objects {
            if tcod.fov.is_in_fov(object.x, object.y) {
                object.draw(&mut tcod.con);
            }
        }

        let fov_recumpute = previous_player_position != (objects[PLAYER].x, objects[PLAYER].y);
        render_all(&mut tcod, &mut game, &objects, fov_recumpute);
        tcod.root.flush();

        let player = &mut objects[PLAYER];
        previous_player_position = (player.x, player.y);

        let player_action = handle_keys(&mut tcod, &game, &mut objects);
        println!("ACTION {:?}", player_action);
        if player_action == PlayerAction::Exit {
            break;
        }

        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for object in &objects {
                if (object as *const _) != (&objects[PLAYER] as *const _) {
                    println!("The {} growls!", object.name);
                }
            }
        }
    }
}
