use ggez::glam::Vec2;
use ggez::{
    audio::{SoundSource, Source},
    event::EventHandler,
    graphics::{Canvas, Color, DrawParam, FontData, Text},
    input::keyboard,
    timer, Context, GameResult,
};

use crate::{tetromino::*, factory::*};

pub struct Game {
  tetromino: Tetromino
}

impl Game {
  pub fn new() -> Self {
    Self { tetromino: Tetromino::new() }
  }
}

impl EventHandler for Game {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    Factory::create();
  }

  fn key_up_event(
    &mut self,
    ctx: &mut Context,
    input: keyboard::KeyInput) -> GameResult {

  }
}