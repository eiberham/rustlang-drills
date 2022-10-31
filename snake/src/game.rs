use ggez::{
    timer,
    graphics::{self, Color, Canvas},
    mint,
    input::keyboard,
    event::{self, EventHandler},
    Context, GameResult
};

use crate::{ snake::*, };

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

        /* let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: 200.0, y: 300.0},
            100.0,
            0.1,
            graphics::Color::WHITE,
        )?; */

        /* let rect = graphics::Rect::new(100.0, 100.0, 16.0, 16.0);

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(rect)
                .color(graphics::Color::WHITE),
        ); */

        self.snake.draw(&mut canvas);

        canvas.finish(ctx)?;

        Ok(())

    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> GameResult {
        match input.keycode {
            Some(keyboard::KeyCode::Up) => {
              println!("Up arrow key has been pressed");
              self.snake.current = Some(Direction::Up)
            }
            Some(keyboard::KeyCode::Left) => {
              println!("Left arrow key has been pressed");
              self.snake.current = Some(Direction::Left)
            }
            Some(keyboard::KeyCode::Right) => {
              println!("Right arrow key has been pressed");
              self.snake.current = Some(Direction::Right)
            }
            Some(keyboard::KeyCode::Down) => {
              println!("Down arrow key has been pressed");
              self.snake.current = Some(Direction::Down)
            }
            Some(keyboard::KeyCode::Escape) => {
              _ctx.request_quit();
            }
            _ => (), // Do nothing
        }
        Ok(())
    }
}