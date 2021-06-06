//! module foo in Rust is either foo.rs or foo/mod.rs
//! use super::foo; is import ..foo
//! (similarly if you add more super:: or dots)
//! use foo::*; is from foo import *

use console_engine::{pixel, screen::Screen, Color, ConsoleEngine, KeyCode, KeyModifiers};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

extern crate rand;
use rand::seq::IteratorRandom;

use crate::dictionary::*;

pub struct Player {
    pub chances: u32,
    pub asserts: Vec<char>,
}

pub struct Config {
    pub height: u32,
    pub width: u32,
    pub fps: u32,
}

pub struct Game {
    pub engine: ConsoleEngine,
    pub screen: Screen,
    pub phrase: String,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.phrase)
    }
}

impl Game {
    pub fn new(config: Config) -> Game {
        Game {
            engine: ConsoleEngine::init(config.width, config.height, config.fps),
            screen: Screen::new(70, 20),
            phrase: "".to_string(),
        }
    }

    pub fn start(&mut self) {
        let title = "The Hangman ∞ Guess or Get Slaughtered";
        self.engine.clear_screen();
        self.engine.set_title(title);

        self.screen.clear();

        self.screen.fill(pixel::pxl_bg(' ', Color::Reset));

        self.screen
            .print_fbg(0, 2, title, Color::Yellow, Color::Reset);

        self.phrase = words::get().expect("fail.");

        self.screen.line(60, 2, 60, 6, pixel::pxl('⛓'));
        self.screen.line(60, 7, 60, 7, pixel::pxl('😰'));

        let size: i32 = self.phrase.len().try_into().unwrap();

        let mut random = rand::thread_rng();

        let mut hint = HashMap::new();

        let clue: String = self
            .phrase
            .chars()
            .choose_multiple(&mut random, 3)
            .iter()
            .map(|val| val.to_string())
            .collect::<String>();

        for x in (0..(size * 5)).step_by(5) {
            self.screen.set_pxl(x, 10, pixel::pxl('_'));
        }

        for (_, y) in clue.as_bytes().iter().enumerate() {
            if let Some(position) = self.index_of(*y) {
                hint.insert(position, *y);
            }
        }

        // print the game screen
        self.engine.print_screen(0, 0, &self.screen);
    }

    pub fn update(&mut self, player: &mut Player) {
        self.engine.wait_frame();

        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        for letter in alphabet.chars() {
            if self.engine.is_key_pressed(KeyCode::Char(letter as char))
                || self.engine.is_key_pressed_with_modifier(
                    KeyCode::Char(letter as char),
                    KeyModifiers::SHIFT,
                )
            {
                if !self.phrase.contains(letter) || !letter.is_ascii_alphabetic() {
                    if player.chances < 1 {
                        // you lose !!
                        // self.start();
                        break;
                    }
                    player.chances -= 1;
                    self.engine.print_fbg(
                        7,
                        4,
                        &player.chances.to_string()[..],
                        Color::White,
                        Color::Reset,
                    );
                    continue;
                }
                if player.asserts.contains(&letter) {
                    continue;
                }
                for (index, character) in self.phrase.char_indices() {
                    if letter.to_lowercase().to_string() == character.to_lowercase().to_string() {
                        self.engine.print(
                            (index * 5).try_into().unwrap(),
                            10,
                            &letter.to_string()[..],
                        );
                        player.asserts.push(letter);
                        if player.asserts.len() == self.phrase.len().try_into().unwrap() {
                            // you win !!
                            let player = Player {
                                asserts: vec![],
                                chances: 5,
                            };
                            self.start();
                            self.stats(&player);
                            break;
                        }
                    }
                }
            }
        }

        self.engine.draw();
    }

    fn index_of(&mut self, character: u8) -> Option<usize> {
        let bytes = self.phrase.as_bytes();
        let position = bytes.iter().position(|&val| val == character);
        match position {
            Some(pos) => Some(pos),
            None => None,
        }
    }

    pub fn stats(&mut self, player: &Player) -> () {
        let chances = "Tries: ".to_owned(); // converts &str to string.
        self.engine.print_fbg(
            0,
            4,
            &(chances.clone() + &player.chances.to_string()),
            Color::White,
            Color::Reset,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_of() {
        let mut game = Game {
            engine: console_engine::ConsoleEngine::init(20, 10, 3),
<<<<<<< HEAD
            screen: Screen::new(20, 11),
            phrase: "test".to_string(),
=======
            screen: Screen::new(20,11),
            phrase: "test".to_string()
>>>>>>> 53c1311c4fb7092ee689b10ac94ca94a4789cb81
        };

        let chr: u8 = 's' as u8;

        assert_eq!(game.index_of(chr).unwrap(), 2);
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> 53c1311c4fb7092ee689b10ac94ca94a4789cb81
