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
    ctx.gfx.add_font("arcade", FontData::from_path(ctx, "/arcade.ttf")?,);
    
    let mut text = Text::new(format!("Game Over !!"));
    text.set_font("arcade").set_scale(32.);
    
    // Draw the game title
    canvas.draw(
        &text,
        DrawParam::from([32., 430.]).color(Color::RED),
    );
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

