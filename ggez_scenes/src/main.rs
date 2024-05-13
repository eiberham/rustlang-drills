mod states;
use crate::states::{
  start::*, play::*, over::*
};

mod scene;
use crate::scene::*;


use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
    audio::{SoundSource, Source},
    event::EventHandler,
    graphics::{Canvas, Color, Text, DrawParam, FontData},
    input::{self, keyboard::{KeyCode, KeyInput}},
    context,
    timer, Context, GameResult
};

pub struct MainState {
	pub stack: SceneStack<ggez::Context>,
}

impl MainState {
  // create a function that would sum two floating numbers
  pub fn new(ctx: &mut Context) -> Self {
    // scene::SceneStack<World, input::InputEvent>;
    let mut stack = SceneStack::new(ctx);
    stack.push(Box::new(StartScene::new()));
    
    Self { stack }
  }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(8) {

      }

      Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.stack.draw(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
      // self.stack.input(_ctx, input);
      Ok(())
    }
}


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("scenes", "eiberham")
        .window_setup(WindowSetup::default().title("scenes ggez"))
        .window_mode(WindowMode::default().dimensions(384.0, 960.0))
        .build()
        .expect("upsss, could not create ggez context!");

    // let c = ggez::Context::from_ctx(ctx);
    let state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, state);
}

