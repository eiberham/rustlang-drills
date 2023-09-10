use ggez::glam::Vec2;
use ggez::{
    audio::{SoundSource, Source},
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam, FontData, Text, Rect},
    input::keyboard,
    timer, Context, GameResult,
};

use crate::{tetromino::*, factory::*, block::*, board::*, bundle::* };

const GAME_FPS: u32 = 8;

#[derive(Debug)]
pub struct Game {
  // The current tetromino respectively
  pub tetromino: Option<Block>,
  // A block bundle that will hold every shape comes up in the game
  // commented out til i find out why this
  pub bundle: Bundle<Block>,
  // A 20x10 array of squares; arrays have a fixed size, known at compile time
  // 20 rows, 10 columns e.g pub board: [[Square; 10]; 20]
  // pub board: [[i32; 10]; 20],
  pub board: Board,
  // the player's score
  pub score: usize
}

impl Game {
  pub fn new() -> Self {
    Self {
      // At the beginning there's no piece
      tetromino: None,
      bundle: Bundle::new(),
      // the inner square brackets are the columns, the outter square brackets are rows.
      board: Board::new(),
      score: 0
    }
  }
}

impl EventHandler for Game {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    while ctx.time.check_update_time(GAME_FPS) {
      // game logic
      if self.tetromino.is_some() {
        let mut block: Block = self.tetromino.unwrap();

        // check if the block position already reached the bottom
        // take into account the difference in height according to
        // the block's rotation
        // check if there's a collision between the block and the bundle blocks
        // if there is then set the tetromino to None.
        if !block.landed() && !block.collides(self.bundle.clone()) {
          block.move_d();
          self.tetromino = Some(block);
        } else {
          // save block titles in bundle
          self.bundle.push(block);
          self.tetromino = None
        }

        // check if the board row has been completed
        // if so then


        timer::sleep(std::time::Duration::from_millis(60));
      }
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

    if self.tetromino.is_none() {
      // gets a random shape i,l,z,o, etc ...
      let shape: Shape = rand::random();
      let piece: Block = Piece::create(shape);
      self.tetromino = Some(piece);
      self.tetromino.unwrap().draw(&mut canvas, ctx)?;
    } else {
      self.tetromino.unwrap().draw(&mut canvas, ctx)?;
    }

    // draws all the blocks saved in the bundle
    if !self.bundle.is_empty() {
      let blocks: Vec<Block> = self.bundle.values.clone();
      for mut block in blocks {
        block.draw(&mut canvas, ctx)?;
      }
    }

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
          let mut block: Block = self.tetromino.unwrap();
          block.move_l();
          self.tetromino = Some(block);
        }
        Some(keyboard::KeyCode::Right) => {
          let mut block: Block = self.tetromino.unwrap();
          block.move_r();
          self.tetromino = Some(block);
        }
        Some(keyboard::KeyCode::Up) => {
          let mut block: Block = self.tetromino.unwrap();
          block.rotate();
          self.tetromino = Some(block);
        }
        _ => (),
      }
      Ok(())
  }
}