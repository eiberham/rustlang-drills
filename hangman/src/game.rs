//! module foo in Rust is either foo.rs or foo/mod.rs
//! use super::foo; is import ..foo
//! (similarly if you add more super:: or dots)
//! use foo::*; is from foo import *

use console_engine::{screen::Screen, ConsoleEngine, Color};

pub struct Config {
    height: u32,
    width:  u32,
    fps:    u32
}

pub struct Game {
    pub engine: ConsoleEngine,
    pub screen: Screen
}

impl Game {
    pub fn new(config: Config) -> Game {
        Game { 
            engine: ConsoleEngine::init(config.width, config.height, config.fps),
            screen: Screen::new(config.width, config.height)
        }
    }

    pub fn init(&mut self) -> () {
        self.engine.clear_screen();
        self.screen.print_fbg(
            (self.screen.get_width() as i32 / 2) / 2 , 
            2, 
            "The Hangman - Guess or die",
            Color::Red, 
            Color::Reset
        );

        // print the game screen
        self.engine.print_screen(1, 0, &self.screen); 
    }

    pub fn update(&mut self){
        self.engine.wait_frame();
        // engine.clear_screen(); // reset the screen
        self.engine.draw(); // draw the screen
    }
}