

pub struct PlayScene {
    done: bool
}

impl Scene for PlayScene {
  fn update(&mut self, gameworld: &mut C, ctx: &mut ggez::Context) -> SceneSwitch<C, Ev> {
    
  }
  
  fn draw(&mut self, gameworld: &mut C, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.finish(ctx)?;
    Ok(())
  }
  
  fn input(&mut self, gameworld: &mut C, event: Ev, started: bool) {
    
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

