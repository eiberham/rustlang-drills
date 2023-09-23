use ggez::{
    audio::{SoundSource, Source},
    event::EventHandler,
    graphics::{Canvas, Color, Text},
    input::keyboard::{KeyCode, KeyInput},
    timer, Context, GameResult
};

use crate::{utils::*, factory::*, block::*, board::*};

const GAME_FPS: u32 = 8;

#[derive(Debug)]
pub struct Game {
  pub block: Option<Block>,
  pub board: Board,
  pub score: usize,
  pub music: Source
}

impl Game {
  pub fn new(ctx: &mut Context) -> Self {
    let mut music = Source::new(ctx, "/music.mp3").unwrap();
    music.set_repeat(true);
    music.play(ctx).unwrap();
    music.set_volume(0.1);

    Self {
      block: None,
      board: Board::new(),
      score: 0,
      music
    }
  }
}

impl EventHandler for Game {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    while ctx.time.check_update_time(GAME_FPS) {
      if self.block.is_some() {
        let mut block: Block = self.block.unwrap();

        // checks if the block has landed
        // checks if there's a collision
        if !block.has_landed() && !block.collides(self.board) {
          block.move_down();
          self.block = Some(block);

          if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            if ctx.keyboard.is_key_repeated() {
              let mut block: Block = self.block.unwrap();
              block.drop();
              self.block = Some(block);
            }
          }

        } else {
          // ocupies position on board
          self.board.fill(block.get_positions(), block.color);
          self.block = None;
        }

        // checks if any row has been filled
        let count = self.board.clear(ctx);
        self.score += count;
      }
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

    if self.block.is_none() {
      let shape: Shape = rand::random();
      let block: Block = Piece::create(shape);
      self.block = Some(block);
      self.block.unwrap().draw(&mut canvas, ctx)?;
    } else {
      self.block.unwrap().draw(&mut canvas, ctx)?;
    }

    // draws the board
    self.board.render(&mut canvas, ctx);

    canvas.finish(ctx)?;
    timer::sleep(std::time::Duration::from_millis(16));
    Ok(())
  }

  fn key_up_event(
    &mut self,
    _ctx: &mut Context,
    input: KeyInput) -> GameResult {
      match input.keycode {
        Some(KeyCode::Left) => {
          let mut block: Block = self.block.unwrap();
          block.move_left();
          self.block = Some(block);
        }
        Some(KeyCode::Right) => {
          let mut block: Block = self.block.unwrap();
          block.move_right();
          self.block = Some(block);
        }
        Some(KeyCode::Up) => {
          let mut block: Block = self.block.unwrap();
          block.rotate();
          self.block = Some(block);
        }
        _ => (),
      }
      Ok(())
  }

}

