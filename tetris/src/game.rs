use ggez::glam::Vec2;
use ggez::{
    audio::{SoundSource, Source},
    event::EventHandler,
    graphics::{Canvas, Color, Text},
    input::keyboard,
    timer, Context, GameResult, GameError
};

use crate::{utils::*, factory::*, block::*, board::*};

const GAME_FPS: u32 = 8;

#[derive(Debug)]
pub struct Game {
  // The current tetromino respectively
  pub tetromino: Option<Block>,
  // A 20x10 array of squares; arrays have a fixed size, known at compile time
  // 20 rows, 10 columns e.g pub board: [[Square; 10]; 20]
  // pub board: [[i32; 10]; 20],
  pub board: Board,
  // the player's score
  pub score: usize,
  pub music: Source
}

impl Game {
  pub fn new(ctx: &mut Context) -> Self {
    let mut music = Source::new(ctx, "/music.mp3").unwrap();
    music.set_repeat(true);
    music.play(ctx).unwrap();
    music.set_volume(0.2);

    Self {
      // At the beginning there's no piece
      tetromino: None,
      // the inner square brackets are the columns, the outter square brackets are rows.
      board: Board::new(),
      score: 0,
      music
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
        // check if there's a collision between the block and the
        // board; if there is then set the tetromino to None.
        if !block.landed() && !block.collides(self.board) {
          block.move_d();
          self.tetromino = Some(block);
        } else {

          // ocupy position on board
          self.board.fill(block.tiles(), block.color);
          self.tetromino = None;
        }

        // check if any row has been filled
        self.board.clear(ctx);
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

    // draws the board
    self.board.render(&mut canvas, ctx);

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