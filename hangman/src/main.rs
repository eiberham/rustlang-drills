mod dictionary;
use dictionary::*;

mod game;
use crate::game::*;

use console_engine::{KeyCode};

fn main() {
    println!("{:?}", words::get());

    let mut conf = Config { height: 20, width: 50, fps: 10 };
    let mut game = Game::new(conf);

    game::init();

    /* screen.print_fbg(
        (screen.get_width() as i32 / 2) / 2 , 
        2, 
        "The Hangman - Guess or die",
        Color::Red, 
        Color::Reset
    );

    // print the game screen
    engine.print_screen(1, 0, &screen);  */   

    loop {
        game.update();
        if game.engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
    };

}
