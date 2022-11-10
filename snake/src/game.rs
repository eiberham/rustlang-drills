use ggez::{
    graphics::{ self, Text, Color },
    input::keyboard,
    event::{ EventHandler },
    Context, GameResult,
    audio::{ Source, SoundSource }
};

use crate::{ snake::*, food::* };

const fps: u32 = 8;

/// Represents the game itself.
/// Handles the game event loop, draws all the characters and is made out
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
          game_over: true,
          level: 0x1
        }
    }

    /// Indicates if the game is already over.
    fn is_over(&self) -> bool {
      self.game_over
    }

    /// Ends the game altogether.
    fn end_game(&mut self, ctx: &mut Context) -> () {
      self.game_over = true;
      self.snake.current_direction = None;
    }

}

/// Implements the EventHandler trait in order to register callbacks for events,
/// and various sub-modules such as graphics and audio
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(fps) {
        self.snake.update(&mut self.food, ctx);

        if !self.is_over() {
          if self.snake.collides() {
            let mut sound = Source::new(ctx, "/finish.wav").unwrap();
            sound.play_detached(ctx).unwrap();

            self.end_game(ctx);

            // draw game over text on the screen
            // press any key to continue
          }
        }
      }

      Ok(())
    }

    /// Draws all the actors in the game.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.snake.draw(&mut canvas);

        self.food.draw(&mut canvas);

        // It looks like off rip new fonts must be added to the resources folder to be able
        // to set them.
        let mut text = Text::new("Game Over");
        // text.set_font("Chalkduster Normal");
        text.set_scale(32.0);
        // When drawing through these calls, `DrawParam` will work as they are documented.
        canvas.draw(
            &text,
            graphics::DrawParam::from([0.0, 0.0]).color(Color::RED),
        );

        canvas.finish(ctx)?;

        Ok(())
    }

    /// Handles the key events
    /// General control guidelines:
    ///
    /// In order to move the snake around use the arrow keys. In order to start the
    /// game press the return key.
    ///
    /// In order to quit press the escape key.
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
        Some(keyboard::KeyCode::Return) => {
          // If the player presses the return key then set game over to falsy and game
          // starts.
          self.game_over = false;

        }
        _ => (), // Do nothing. Any other pressed keys will be ignored at this point.
      }
      Ok(())
    }
}