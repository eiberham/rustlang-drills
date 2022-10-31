mod game;
use crate::game::*; // bring all public stuff into scope

/* mod board;
use crate::board::*; */

mod snake;
mod food;

use ggez::{
    event,
    ContextBuilder
};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("snake", "eiberham")
        .window_setup(ggez::conf::WindowSetup::default().title("snake game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1200.0, 800.0))
        .build()
        .expect("upsss, could not create ggez context!");

    // Setup the board dimensions
    // let board = Board::new((30, 20), (32, 32));

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = Game::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}
