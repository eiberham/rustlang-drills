use ggez::{
  audio::{SoundSource, Source},
  graphics::{Canvas, Color, Text, DrawParam, FontData, Image, Rect},
  input::keyboard::{KeyCode, KeyInput},
  Context
};

use crate::scene::*;

pub struct StartScene {
  done: bool
}

impl StartScene {
  pub fn new() -> Self {
    Self { done: false }
  }
  
  pub fn draw_text(
    &mut self,
    canvas: &mut Canvas, 
    text: &str, 
    point: [f32; 2], 
    scale: f32, 
    color: Color) -> () {
      let mut text = Text::new(format!("{}", text));
      text.set_font("arcade").set_scale(scale);
      canvas.draw(
          &text,
          DrawParam::from(point).color(color),
      );
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
    let background = Image::from_path(ctx, "/logo.png").unwrap();
    
    ctx.gfx.add_font("arcade", FontData::from_path(ctx, "/arcade.ttf")?,);
    
    self.draw_text(&mut canvas, "Tetris", [64., 430.], 40., Color::RED);
    
    self.draw_text(&mut canvas, "Press Enter Key", [64., 580.], 18., Color::WHITE);
    
    // Draw the top image
    canvas.draw(
        &background,
        DrawParam::new()
          .src(Rect::new(0., 0., 1., 1.))
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

