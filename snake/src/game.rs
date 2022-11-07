use ggez::{
    timer,
    graphics::{self, Color, Canvas},
    mint,
    input::keyboard,
    event::{self, EventHandler},
    Context, GameResult
};

use crate::{ snake::*, food::* };

const fps: u32 = 8;

pub struct Game {
    pub snake: Snake,
    pub food: Food,
    pub game_over: bool,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        // Load/create resources such as images here.

        // Create here new instances of the game entities.

        let snake = Snake::new();
        let food = Food::new();

        Game {
          snake,
          food,
          game_over: false,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(fps) {
        self.snake.update(&mut self.food, ctx)
      }

      Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        // Draw code here...

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // Here I'm supposed to draw the snake and the food.
        // The snake is going to have its own draw method.

        self.snake.draw(&mut canvas);

        self.food.draw(&mut canvas);

        canvas.finish(ctx)?;

        Ok(())

    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> GameResult {
      match input.keycode {
        Some(keyboard::KeyCode::Up) => {
          if !matches!(self.snake.previous, Direction::D ) {
            self.snake.divert(Direction::U);
          }
        }
        Some(keyboard::KeyCode::Left) => {
          if !matches!(self.snake.previous, Direction::R ) {
            self.snake.divert(Direction::L);
          }
        }
        Some(keyboard::KeyCode::Right) => {
          if !matches!(self.snake.previous, Direction::L ) {
            self.snake.divert(Direction::R);
          }
        }
        Some(keyboard::KeyCode::Down) => {
          if !matches!(self.snake.previous, Direction::U ) {
            self.snake.divert(Direction::D);
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