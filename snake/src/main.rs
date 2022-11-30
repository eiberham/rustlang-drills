//! This is the main file, where the entry point of the game is.

extern crate smart_default;

mod game;
use crate::game::*; // bring all public stuff into scope

mod snake;
mod food;
mod tile;
mod scene;

use ggez::{
    conf::{ WindowSetup, WindowMode },
    event,
    ContextBuilder
};
use std::path;

fn main() {
    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("snake", "eiberham")
        .add_resource_path(path::PathBuf::from("./resources"))
        .window_setup(WindowSetup::default().title("snake game"))
        .window_mode(WindowMode::default().dimensions(960.0, 960.0))
        .build()
        .expect("upsss, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = Game::new();

    // Run!
    event::run(ctx, event_loop, game);
}
