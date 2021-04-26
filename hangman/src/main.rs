mod dictionary;
use dictionary::*;

use console_engine::{screen::Screen, ConsoleEngine, Color, KeyCode};

fn main() {
    const SCREEN_WIDTH: u32 = 50;
    const SCREEN_HEIGHT: u32 = 20;
    const TARGET_FPS: u32 = 10;

    println!("{:?}", words::get());

    let mut engine = ConsoleEngine::init(SCREEN_WIDTH, SCREEN_HEIGHT, TARGET_FPS);

    let mut screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    screen.print_fbg(
        (screen.get_width() as i32 / 2) / 2 , 
        2, 
        "The Hangman - Guess or die",
        Color::Red, 
        Color::Reset
    );

    // print the game screen
    engine.print_screen(1, 0, &screen);    

    loop {
        engine.wait_frame();
        // engine.clear_screen(); // reset the screen

        if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }

        engine.draw(); // draw the screen
    };

}
