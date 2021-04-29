mod dictionary;
//use dictionary::*;

mod game;
use crate::game::*;

use console_engine::{KeyCode};

fn main() {
    // println!("{:?}", words::get());

    let conf: Config = Config { height: 20, width: 50, fps: 10 };

    let mut game = Game::new(conf);

    println!("game: {}", game);

    // let player = Player { chances: 7 };

    game.start();

    loop {
        game.update();
        if game.engine.is_key_pressed(KeyCode::Esc) { // if the user presses escape
            break; // exits app
        }
    };

}
