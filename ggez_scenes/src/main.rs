use ggez::{
    graphics::{ Color, Canvas },
    conf::{ WindowSetup, WindowMode },
    input::keyboard,
    event::{ self, EventHandler },
    ContextBuilder,
    Context,
    GameResult
};

pub enum State {
  Start,
  Running,
  Pause
}

/// A trait for you to implement on a scene.
/// Defines the callbacks the scene uses:
/// a common context type `C`, and an input event type `Ev`.
pub trait Scene<C, Ev> {
    fn update(&mut self, ctx: &mut ggez::Context) -> SceneSwitch<C, Ev>;
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()>;
    fn input(&mut self, event: Ev, started: bool);
    /// Only used for human-readable convenience (or not at all, tbh)
    fn name(&self) -> &str;
    /// This returns whether or not to draw the next scene down on the
    /// stack as well; this is useful for layers or GUI stuff that
    /// only partially covers the screen.
    fn draw_previous(&self) -> bool {
        false
    }
}

pub struct MainScene {
    pub scene: State,
	pub stack: Vec<Context>
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            scene: State::Start,
            stack: Vec::new()
        }
    }

	pub fn switch(&mut self, new_state: State) -> () {

	}
}

impl EventHandler for MainScene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(8) {

      }

      Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        match self.scene  {
            State::Start => {

            }
            State::Running => {

            }
            State::Pause => {

            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> GameResult {
      match input.keycode {
        Some(keyboard::KeyCode::Return) => {
          self.switch(State::Running);
        }
        Some(keyboard::KeyCode::Escape) => {
          _ctx.request_quit();
        }
        _ => (),
      }
      Ok(())
    }
}


fn main() {
    let (ctx, event_loop) = ContextBuilder::new("scenes", "eiberham")
        .window_setup(WindowSetup::default().title("scenes ggez"))
        .window_mode(WindowMode::default().dimensions(960.0, 960.0))
        .build()
        .expect("upsss, could not create ggez context!");

    let game = MainScene::new();

    event::run(ctx, event_loop, game);
}

