use ggez::{
  glam::Vec2,
  audio::{SoundSource, Source},
  event::EventHandler,
  graphics::{Canvas, Color, Text, DrawParam, FontData},
  input::keyboard::{KeyCode, KeyInput},
  timer, Context, GameResult
};

pub struct StartScene {
  done: bool
}

impl Scene for StartScene {
  fn update(&mut self, gameworld: &mut C, ctx: &mut ggez::Context) -> SceneSwitch<C, Ev> {
    Ok(())
  }
  
  fn draw(&mut self, gameworld: &mut C, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      Vec2::new(0.0, 0.0),
      100.0,
      2.0,
      Color::WHITE,
    )?;
    canvas.draw(&circle, Vec2::new(0., 0.));
  }
  
  fn input(&mut self, gameworld: &mut C, event: Ev, started: bool) {
    
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

