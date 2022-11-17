//! Game abstraction.
//!
//! Provides an abstraction over the game. It is supposed to handle the
//! game events and initial graphics.
//!
use std::thread::sleep;
use std::time::Duration;
use ggez::glam::Vec2;
use ggez::{
    graphics::{ self, Text, Color },
    input::keyboard,
    event::{ EventHandler },
    Context, GameResult,
    audio::{ Source, SoundSource }
};

use crate::{ snake::*, food::* };

const GAME_FPS: u32 = 8;

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
    pub level: u16,
    pub score: u16,
    pub milestones: Vec<u16>,
    pub music: Source
}

impl Game {
    /// Creates new instances of the game's actors.
    pub fn new(ctx: &mut Context) -> Game {
      let music = Source::new(ctx, "/music.wav").unwrap();
      music.play_later().unwrap();
      Game {
        snake: Snake::new(),
        food: Food::new(),
        game_over: false,
        level: 0x1,
        // the score is a unsigned 16-bit scalar so
        // its limited to go from zero up to 65535.
        score: 0x0,
        // reaching each milestone makes the game
        // level go up, therefore the game levels
        // are only six.
        milestones: vec![ 8, 16, 24, 32, 40, 48, 56, 64 ],
        music
      }
    }

    /// Tells if the game is already over.
    fn is_over(&self) -> bool {
      self.game_over
    }

    /// Ends the game altogether. Keeps the snake in a steady position and
    /// restarts the game back again five seconds later.
    fn end_game(&mut self) -> () {
      self.game_over = true;
      self.snake.current_direction = None;
      sleep(Duration::from_millis(5000));
      self.snake.current_direction = Some(Direction::R);
      self.restart();
    }

    /// Levels up.
    fn level_up(&mut self) -> () {
      self.level += 1;
    }

    /// Scores up.
    fn score_up(&mut self) -> () {
      self.score += 1;
    }

    /// Draws the game stats over the canvas in the given point coordinates.
    fn draw_scorekeeping(
      &mut self,
      canvas: &mut graphics::Canvas,
      string: &str,
      point: Vec2,
      value: u16
    ) -> () {
        let mut text = Text::new(format!("{}: {}", string, value));
        text.set_font("Arcade")
            .set_scale(24.0);

        canvas.draw(
            &text,
            graphics::DrawParam::from(point).color(Color::WHITE),
        );

    }

    /// Resets the game altogether.
    fn restart(&mut self) {
        self.snake = Snake::new();
        self.food = Food::new();
        self.game_over = false;
        self.score = 0x0;
        self.level = 0x1;
    }

}

/// Implements the EventHandler trait in order to register callbacks for events,
/// and various sub-modules such as graphics and audio
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(GAME_FPS) {
        self.snake.update(&mut self.food, ctx);

        if !self.is_over() {
          if self.snake.collides() {
            let mut sound = Source::new(ctx, "/finish.wav").unwrap();
            sound.play_detached(ctx).unwrap();

            self.end_game();
          }
        }

        if self.snake.ate.is_some() {
          self.score_up();
          self.snake.ate = None;
          if self.milestones.iter().any(|&x| x == self.score) {
            self.level_up()
          }
        }

      }

      Ok(())
    }

    /// Draws all the actors in the game.
    ///
    /// It looks like off rip new fonts must be added to the resources folder to be able
    /// to set them.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.snake.draw(&mut canvas);

        self.food.draw(&mut canvas);

        ctx.gfx.add_font(
            "Arcade",
            graphics::FontData::from_path(ctx, "/arcade.ttf")?,
        );

        self.draw_scorekeeping(
          &mut canvas,
          "level",
          Vec2::new(32.0, 32.0),
          self.level
        );

        self.draw_scorekeeping(
          &mut canvas,
          "score",
          Vec2::new(720.0, 32.0),
          self.score
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
        _ => (),
      }
      Ok(())
    }
}