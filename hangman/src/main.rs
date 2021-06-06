mod dictionary;
//use dictionary::*;

mod game;
use crate::game::*;

use console_engine::{KeyCode, KeyModifiers};

fn main() {
    let conf: Config = Config {
        height: 30,
        width: 100,
        fps: 10,
    };
    let mut game = Game::new(conf);

    let mut player = Player {
        chances: 5,
        asserts: vec![],
    };

    game.start();
    game.stats(&player);

    loop {
        game.update(&mut player);

        if game
            .engine
            .is_key_pressed_with_modifier(KeyCode::Char('c'), KeyModifiers::CONTROL)
        {
            break;
        }

        if game.engine.is_key_pressed(KeyCode::Esc) {
            // if the user presses escape
            break;
        }
    }
}
