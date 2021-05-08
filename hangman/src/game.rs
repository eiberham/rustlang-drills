//! module foo in Rust is either foo.rs or foo/mod.rs
//! use super::foo; is import ..foo
//! (similarly if you add more super:: or dots)
//! use foo::*; is from foo import *

use console_engine::{screen::Screen, ConsoleEngine, Color};
use std::fmt;
use std::iter::FromIterator;
use std::str;

extern crate rand;
   use rand::seq::IteratorRandom;

//use rand::{self, Rng, IteratorRandom};


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

        self.screen.print_fbg(
            (self.screen.get_width() as i32 / 2) / 2 , 
            2, 
            title,
            Color::Red, 
            Color::Reset
        );

        let size: i32 = self.phrase.len() as i32;

        let mut rng = rand::thread_rng();

        // let clues : Vec<char> = self.word.chars().choose_multiple(&mut rng, 3);
        // gets three random values from the word in order to help the player.
        let clues : Vec<String> = self.phrase
        .chars()
        .choose_multiple(&mut rng, 3)
        .iter()
        .map(|val| val.to_string())
        .collect();

        self.screen.print_fbg(1, 12, &String::from_iter(clues), Color::Red, Color::Reset);

        for x in (1..(size * 5)).step_by(5) {
            self.screen.print_fbg(x, 10, " ___ ", Color::Red, Color::Reset);
        }

        for y in clues.iter() {
            let pos = self.position(y);
            self.screen.print_fbg(1, 14, &pos.to_string()[..], Color::Red, Color::Reset);
        }

        // print the game screen
        self.engine.print_screen(1, 0, &self.screen); 
    }

    pub fn update(&mut self){
        self.engine.wait_frame();
        // engine.clear_screen(); // reset the screen
        self.engine.draw(); // draw the screen
    }

    fn position(&mut self, character: &String) -> usize {
        let bytes = self.phrase.as_bytes();
        let position = bytes.iter().enumerate().position(|(index, &val)| val.to_string() == *character);
        match position {
            Some(pos) => pos,
            None => panic!("we are fucked up")
        }
        
    }
}