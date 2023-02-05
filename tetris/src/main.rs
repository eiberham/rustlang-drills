//! This is the main file, where the entry point of the game is.

extern crate smart_default;

mod game;
use crate::game::*;

mod factory;
mod shapes;
mod tetromino;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};
use std::path;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("tetris", "eiberham")
        .add_resource_path(path::PathBuf::from("./resources"))
        .window_setup(WindowSetup::default().title("tetris game"))
        .window_mode(WindowMode::default().dimensions(960.0, 960.0))
        .build()
        .expect("upsss, could not create ggez context!");

    let game = Game::new();
    event::run(ctx, event_loop, game);
}
