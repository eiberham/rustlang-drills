use ggez::{
    timer,
    graphics::{self, Color, Canvas},
    mint,
    input::keyboard,
    event::{self, EventHandler},
    Context, GameResult
};

use crate::{ snake::*, food::*};

const fps: u32 = 8;

pub struct Game {
    pub snake: Snake,
    pub over: bool,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        // Load/create resources such as images here.

        // Create here new instances of the game entities.

        let snake = Snake::new();
        let food = Food::new();

        Game {
          snake,
          over: false,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(fps) {
        self.snake.update()
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
              if !matches!(self.snake.previous, Direction::Down ) {
                self.snake.current = Some(Direction::Up);
                self.snake.previous = Direction::Up;
              }
            }
            Some(keyboard::KeyCode::Left) => {
              if !matches!(self.snake.previous, Direction::Right ) {
                self.snake.current = Some(Direction::Left);
                self.snake.previous = Direction::Left;
              }
            }
            Some(keyboard::KeyCode::Right) => {
              if !matches!(self.snake.previous, Direction::Left ) {
                self.snake.current = Some(Direction::Right);
                self.snake.previous = Direction::Right;
              }
            }
            Some(keyboard::KeyCode::Down) => {
              if !matches!(self.snake.previous, Direction::Up ) {
                self.snake.current = Some(Direction::Down);
                self.snake.previous = Direction::Down;
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