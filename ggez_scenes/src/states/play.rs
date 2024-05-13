

use ggez::{
  glam::Vec2,
  audio::{SoundSource, Source},
  event::EventHandler,
  graphics::{self, Canvas, Color, Text, DrawParam, FontData},
  input::keyboard::{KeyCode, KeyInput},
  timer, Context, GameResult
};

use crate::scene::*;

pub struct PlayScene {
    done: bool
}

impl<Ev> Scene<Ev> for PlayScene {
  fn update(&mut self, ctx: &mut ggez::Context) -> SceneSwitch<Ev> {
    SceneSwitch::None
  }
  
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.finish(ctx)?;
    Ok(())
  }
  
  fn input(&mut self, event: Ev, started: bool) -> () {
    
  }
  
  fn name(&self) -> &str {
    return "play"
  }
  
  /// This returns whether or not to draw the next scene down on the
  /// stack as well; this is useful for layers or GUI stuff that
  /// only partially covers the screen.
  fn draw_previous(&self) -> bool {
      false
  }
}

