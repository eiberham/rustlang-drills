use ggez::{
    graphics::{ self },
    input::keyboard,
    event::{ EventHandler },
    Context, GameResult
};

use crate::{ snake::*, food::* };

const fps: u32 = 8;

/// Represents the game itself.
/// Handles the game event loop, draws all the characters and is composed
/// of the following entities:
///
/// Snake: the main character.
///
pub struct Game {
    pub snake: Snake,
    pub food: Food,
    pub game_over: bool,
    pub level: u8
}

impl Game {
    /// Creates new instances of the game's actors.
    pub fn new(ctx: &mut Context) -> Game {
        // Load/create resources such as images here.

        let snake = Snake::new();

        let food = Food::new();

        Game {
          snake,
          food,
          game_over: false,
          level: 1
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(fps) {
        self.snake.update(&mut self.food, ctx);
        if self.snake.collides() {
          self.game_over = true;
          self.snake.current_direction = None;
        }
      }

      Ok(())
    }

    /// Draws all the actors in the game.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.snake.draw(&mut canvas);

        self.food.draw(&mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> GameResult {
      match input.keycode {
        Some(keyboard::KeyCode::Up) => {
          if !matches!(self.snake.previous, Direction::D ) {
            self.snake.change_direction(Direction::U);
          }
        }
        Some(keyboard::KeyCode::Left) => {
          if !matches!(self.snake.previous, Direction::R ) {
            self.snake.change_direction(Direction::L);
          }
        }
        Some(keyboard::KeyCode::Right) => {
          if !matches!(self.snake.previous, Direction::L ) {
            self.snake.change_direction(Direction::R);
          }
        }
        Some(keyboard::KeyCode::Down) => {
          if !matches!(self.snake.previous, Direction::U ) {
            self.snake.change_direction(Direction::D);
          }
        }
        Some(keyboard::KeyCode::Escape) => {
          _ctx.request_quit();
        }
        _ => (), // Do nothing
      }
      Ok(())
    }
}