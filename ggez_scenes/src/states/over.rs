use ggez::{
  glam::Vec2,
  audio::{SoundSource, Source},
  event::EventHandler,
  graphics::{self, Canvas, Color, Text, DrawParam, FontData},
  input::keyboard::{KeyCode, KeyInput},
  timer, Context, GameResult
};

use crate::scene::*;

pub struct OverScene {
  done: bool
}

impl OverScene {
  pub fn new() -> Self {
      Self { done: false }
    }
}

impl<Ev> Scene<Ev> for OverScene {
  fn update(&mut self, ctx: &mut ggez::Context) -> SceneSwitch<Ev> {
    SceneSwitch::None
  }
  
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      Vec2::new(0.0, 0.0),
      100.0,
      2.0,
      Color::BLUE,
    )?;
    canvas.draw(&circle, Vec2::new(0., 0.));
    canvas.finish(ctx)?;
    Ok(())
  }
  
  fn input(&mut self, ctx: &mut Context, input: KeyInput) -> () {

  }
  
  fn name(&self) -> &str {
    return "over"
  }
  
  /// This returns whether or not to draw the next scene down on the
  /// stack as well; this is useful for layers or GUI stuff that
  /// only partially covers the screen.
  fn draw_previous(&self) -> bool {
    false
  }
}

