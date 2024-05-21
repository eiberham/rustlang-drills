use ggez::{
  glam::Vec2,
  audio::{SoundSource, Source},
  event::EventHandler,
  graphics::{self, Canvas, Color, Text, DrawParam, FontData, Image, Rect},
  input::keyboard::{KeyCode, KeyInput},
  timer, Context, GameResult
};

use crate::{scene::*, PlayScene};

pub struct StartScene {
  done: bool
}

impl StartScene {
  pub fn new() -> Self {
      Self { done: false }
    }
}

impl<Ev> Scene<Ev> for StartScene {
  fn update(&mut self, ctx: &mut ggez::Context) -> SceneSwitch<Ev> {
    if self.done {
        SceneSwitch::Pop
    } else {
        SceneSwitch::None
    }
  }
  
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    let background = Image::from_path(ctx, "/tetris.png").unwrap();
    
    ctx.gfx.add_font("arcade", FontData::from_path(ctx, "/arcade.ttf")?,);
    
    let mut text = Text::new(format!("Tetris"));
    text.set_font("arcade").set_scale(40.);
    
    // Draw the game title
    canvas.draw(
        &text,
        DrawParam::from([64., 430.]).color(Color::RED),
    );
    
    // Draw the top image
    canvas.draw(
        &background,
        DrawParam::new()
          .src(Rect::new(0., 0., 1., 0.5))
          .dest([0., 0.])
    );
        
    canvas.finish(ctx)?;
    Ok(())
  }
  
  fn input(&mut self, ctx: &mut Context, input: KeyInput) -> () {
    match input.keycode {
      Some(KeyCode::Return) => {
          println!("return has been pressed");
          self.done = true;
      }
      _ => (),
    }
  }
  
  fn name(&self) -> &str {
    return "start"
  }
  
  /// This returns whether or not to draw the next scene down on the
  /// stack as well; this is useful for layers or GUI stuff that
  /// only partially covers the screen.
  fn draw_previous(&self) -> bool {
    false
  }
}

