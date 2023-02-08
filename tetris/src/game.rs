use ggez::glam::Vec2;
use ggez::{
    audio::{SoundSource, Source},
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam, FontData, Text, Rect},
    input::keyboard,
    timer, Context, GameResult,
};

use crate::{tetromino::*, factory::*};

use crate::shapes::{i::*};

const GAME_FPS: u32 = 8;

pub struct Game {
  pub tetromino: Option<Box<dyn Tetromino>>
}

impl Game {
  pub fn new() -> Self {
    Self {
      tetromino: None
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
    let shape: Shape = rand::random();

    /* let rectangle = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0.0, 0.0, 64.0, 128.0),
        Color::WHITE,
    )?;

    canvas.draw(&rectangle, DrawParam::new()); */

    match shape {
      Shape::I => {
        let mut piece: Box<dyn Tetromino> = I::create();
        piece.draw(&mut canvas, ctx )
          .expect("Failed to render");
        self.tetromino = Some(piece);
      }
      Shape::O => {

      }
      Shape::T => {

      }
      Shape::S => {

      }
      Shape::Z => {

      }
      Shape::J => {

      }
      Shape::L => {

      }
    }
    // Factory::create(Shape::default());

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