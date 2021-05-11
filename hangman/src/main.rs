mod dictionary;
//use dictionary::*;

mod game;
use crate::game::*;

use console_engine::{KeyCode};

fn main() {

    let conf: Config = Config { height: 30, width: 100, fps: 10 };
    let mut game = Game::new(conf);
    let player = Player { chances: 7 };

    game.start();

    loop {

        game.update();

        if game.engine.is_key_pressed(KeyCode::Esc) { // if the user presses escape
            break; // exits app
        }
    };

}
