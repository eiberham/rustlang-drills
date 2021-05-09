//! module foo in Rust is either foo.rs or foo/mod.rs
//! use super::foo; is import ..foo
//! (similarly if you add more super:: or dots)
//! use foo::*; is from foo import *

use console_engine::{screen::Screen, ConsoleEngine, Color, pixel};
use std::fmt;
use std::collections::HashMap;
use std::convert::TryInto;

extern crate rand;
use rand::seq::IteratorRandom;

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
    phrase: String,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.phrase)
    }
}

impl Game {
    pub fn new(config: Config) -> Game {
        let phrase = words::get().expect("fail.");
        
        Game { 
            engine: ConsoleEngine::init(config.width, config.height, config.fps),
            screen: Screen::new(config.width, config.height),
            phrase: phrase
        }
    }

    pub fn start(&mut self) {
        let title = "The Hangman - Guess or die";
        self.engine.clear_screen();
        self.engine.set_title(title);

        self.screen.fill(pixel::pxl_bg('.', Color::Reset));

        self.screen.print_fbg(
            (self.screen.get_width() as i32 / 2) / 2 , 
            2, 
            title,
            Color::Yellow, 
            Color::Reset
        );

        let size: i32 = self.phrase.len().try_into().unwrap();

        let mut random = rand::thread_rng();

        let mut hint = HashMap::new();

        let clue : String = self.phrase
        .chars()
        .choose_multiple(&mut random, 3)
        .iter()
        .map(|val| val.to_string())
        .collect::<String>();

        self.screen.print_fbg(1, 12, &clue , Color::Red, Color::Reset);

        for x in (5..(size * 5)).step_by(5) {
            self.screen.set_pxl(x, 10, pixel::pxl('🦀'));
        }

        for (x, y) in clue.as_bytes().iter().enumerate() {
            let position = self.position(*y);
            hint.insert(position, *y);
            self.screen.print_fbg(x.try_into().unwrap(), 14, &position.to_string()[..], Color::Red, Color::Reset);
        }

        for (key, value) in hint.iter() {
            self.screen.set_pxl((*key *5).try_into().unwrap(), 10, pixel::pxl(*value  as char));
        }

        // print the game screen
        self.engine.print_screen(0, 0, &self.screen); 
    }

    pub fn update(&mut self){
        self.engine.wait_frame();
        // engine.clear_screen(); // reset the screen
        self.engine.draw(); // draw the screen
    }

    fn position(&mut self, character: u8) -> usize {
        let bytes = self.phrase.as_bytes();
        let position = bytes.iter().position(|&val| val == character);
        match position {
            Some(pos) => pos,
            None => panic!("we are fucked up")
        }
    }
}