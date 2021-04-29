//! module foo in Rust is either foo.rs or foo/mod.rs
//! use super::foo; is import ..foo
//! (similarly if you add more super:: or dots)
//! use foo::*; is from foo import *

use console_engine::{screen::Screen, ConsoleEngine, Color};
use std::fmt;

use crate::dictionary::*;

/* pub struct Player {
    pub chances: u32
} */

pub struct Config {
    pub height: u32,
    pub width:  u32,
    pub fps:    u32
}

pub struct Game {
    pub engine: ConsoleEngine,
    pub screen: Screen,
    word: String,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.word)
    }
}

impl Game {
    pub fn new(config: Config) -> Game {
        let word = words::get().expect("fail.");
        
        Game { 
            engine: ConsoleEngine::init(
                config.width, config.height, config.fps),
            screen: Screen::new(config.width, config.height),
            word:   word
        }
    }

    pub fn start(&mut self) {
        let title = "The Hangman - Guess or die";
        self.engine.clear_screen();
        self.engine.set_title(title);

        self.screen.print_fbg(
            (self.screen.get_width() as i32 / 2) / 2 , 
            2, 
            title,
            Color::Red, 
            Color::Reset
        );

        let size: i32 = self.word.len() as i32;

        for x in (1..(size * 5)).step_by(5) {
            self.screen.print_fbg(x, 10, " ___ ", Color::Red, Color::Reset);
        }

        // print the game screen
        self.engine.print_screen(1, 0, &self.screen); 
    }

    pub fn update(&mut self){
        self.engine.wait_frame();
        // engine.clear_screen(); // reset the screen
        self.engine.draw(); // draw the screen
    }
}