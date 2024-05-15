use rust_roguelike::input::*;
use rust_roguelike::model::*;
use rust_roguelike::renderer::*;
use tcod::colors::*;
use tcod::console::*;

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/litcod tutorial")
        .init();

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod { root, con };

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(0, 0, '@', WHITE);

    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '&', YELLOW);

    let mut objects = [player, npc];

    let game = Game::new(&mut objects[0]);

    while !tcod.root.window_closed() {
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        render_all(&mut tcod, &game, &objects);
        tcod.root.flush();

        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}
