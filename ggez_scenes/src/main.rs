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

use std::path;

pub struct MainState {
	pub stack: SceneStack<ggez::Context>,
}

impl MainState {
  // create a function that would sum two floating numbers
  pub fn new(ctx: &mut Context) -> Self {
    let mut stack: SceneStack<Context> = SceneStack::new(ctx);
    stack.push(Box::new(OverScene::new()));
    stack.push(Box::new(PlayScene::new()));
    stack.push(Box::new(StartScene::new()));
    
    Self { stack }
  }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(8) {
        self.stack.update(ctx);
      }

      Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.stack.draw(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> GameResult {
      // Idea: to send whatever key was pressed along to the current scene to ber handled on the input event
      self.stack.input(ctx, input); 
      Ok(())
    }
}


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("scenes", "eiberham")
        .add_resource_path(path::PathBuf::from("./resources"))
        .window_setup(WindowSetup::default().title("scenes ggez"))
        .window_mode(WindowMode::default().dimensions(384.0, 960.0))
        .build()
        .expect("upsss, could not create ggez context!");

    let state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, state);
}

