#![recursion_limit = "250"]
//! This is the main file, where the entry point of the game is.

extern crate smart_default;

mod game;
use crate::game::*;

mod factory;
mod block;
mod tetromino;
mod squares;
mod board;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};
use std::path;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("tetris", "eiberham")
        .add_resource_path(path::PathBuf::from("./resources"))
        .window_setup(
            WindowSetup::default()
                .title("tetris")
                .icon("/icon.png")
        )
        .window_mode(WindowMode::default().dimensions(384.0, 960.0))
        .build()
        .expect("upsss, could not create ggez context!");

    let game = Game::new();
    event::run(ctx, event_loop, game);
}
