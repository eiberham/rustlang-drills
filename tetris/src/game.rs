use ggez::glam::Vec2;
use ggez::{
    audio::{SoundSource, Source},
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam, FontData, Text, Rect},
    input::keyboard,
    timer, Context, GameResult,
};

use crate::{tetromino::*, factory::*, block::*};

const GAME_FPS: u32 = 8;

pub struct Game {
  // The current tetromino respectively
  pub tetromino: Option<Box<dyn Tetromino>>,
  // The incoming tetromino if any
  pub next_tetromino: Option<Box<dyn Tetromino>>,
  // A 20x10 array of squares; arrays have a fixed size, known at compile time
  // 20 rows, 10 columns e.g pub board: [[Square; 10]; 20]
  pub board: [[i32; 10]; 20],
  // the player's score
  pub score: usize
}

impl Game {
  pub fn new() -> Self {
    Self {
      tetromino: None,
      next_tetromino: None,
      // the inner square brackets are the columns, the outter square brackets are rows.
      board: [[0; 10]; 20],
      score: 0
    }
  }
}

impl EventHandler for Game {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    while ctx.time.check_update_time(GAME_FPS) {
      // game logic
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

    if self.tetromino.is_none() {
      let shape: Shape = rand::random();
      self.tetromino = Some(Builder::create(shape));

      match &self.tetromino {
        Some(block) => {
          let mut tetromino = block.clone();
          tetromino.draw(&mut canvas, ctx)?;
        },
        _ => unreachable!()
      };
    }

    println!("{:?}", self.board);

    canvas.finish(ctx)?;
    timer::sleep(std::time::Duration::from_millis(16));
    Ok(())
  }

  fn key_up_event(
    &mut self,
    ctx: &mut Context,
    input: keyboard::KeyInput) -> GameResult {
      match input.keycode {
        Some(keyboard::KeyCode::Left) => {
          println!("pressed left key");
        }
        Some(keyboard::KeyCode::Right) => {
          println!("pressed right key");
        }
        _ => (),
      }
      Ok(())
  }
}