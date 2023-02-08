//! Game abstraction.
//!
//! Provides an abstraction over the game. It is supposed to handle the
//! game events and initial graphics.
//!

use std::thread::sleep;
use std::time::Duration;
use ggez::glam::Vec2;
use ggez::{
    timer,
    graphics::{ Text, Color, DrawParam, Canvas, FontData },
    input::keyboard,
    event::{ EventHandler },
    Context, GameResult,
    audio::{ Source, SoundSource }
};

use crate::{ snake::*, food::*, scene::* };

const GAME_FPS: u32 = 8;

/// Represents the game itself.
/// Handles the game event loop, draws all the characters and is made out
/// of the following entities:
///
/// Snake: the main character.
///
#[derive(smart_default::SmartDefault)]
pub struct Game {
    // Represents the main character of the entire
    // game.
    #[default(Snake::new())]
    pub snake: Snake,
    // Represents the foodstuff needed for the
    // snake to grow.
    #[default(Food::new())]
    pub food: Food,
    // Indicates whether the game is over or not.
    #[default = false]
    pub game_over: bool,
    // Indicates the game level. It goes from one
    // up to 8.
    #[default = 1 ]
    pub level: u16,
    // the score is a unsigned 16-bit scalar so
    // its limited to go from zero up to 65535.
    #[default = 0 ]
    pub score: u16,
    // reaching each milestone makes the game
    // level go up
    #[default(vec![ 8, 16, 24, 32, 40, 48, 56, 64 ])]
    pub milestones: Vec<u16>,
    // Represents the actual scene
    #[default(Scene::new())]
    pub scene: Scene
}

impl Game {
    pub fn new() -> Game {
      Game::default()
    }

    /// Tells if the game is already over.
    fn is_over(&self) -> bool {
      self.game_over
    }

    /// Ends the game altogether. Keeps the snake in a steady position and
    /// restarts the game back again five seconds later.
    fn end_game(&mut self, ctx: &mut Context) -> () {
      self.game_over = true;
      self.snake.current_direction = None;
      sleep(Duration::from_millis(5000));
      self.scene.change(State::Over);
      self.snake.current_direction = Some(Direction::R);

      ctx.gfx.begin_frame();
      self.draw(ctx);
      ctx.gfx.end_frame();
    }

    /// Levels up.
    fn level_up(&mut self) -> () {
      self.level += 1;
    }

    /// Scores up.
    fn score_up(&mut self) -> () {
      self.score += 1;
    }

    /// Draws the game stats over the canvas.
    fn draw_score(
      &mut self,
      canvas: &mut Canvas,
      string: &str,
      point: Vec2,
      value: u16
    ) -> () {
        let mut text = Text::new(format!("{}: {}", string, value));
        text.set_font("Arcade")
            .set_scale(24.0);
        canvas.draw(
            &text,
            DrawParam::from(point).color(Color::WHITE),
        );
    }

    /// Resets the game altogether.
    fn restart(&mut self) {
        *self = Game::default();
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
            self.end_game(ctx);
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
    /// Off rip new fonts must be added to the resources folder to be able
    /// to set them.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        // TODO:
        // Compile the game for windows
        // Create a text manager crate to handle all the text drawing within the game
        // Try to run the game in the browser using wasm bindings

        ctx.gfx.add_font(
                "Arcade",
                FontData::from_path(ctx, "/arcade.ttf")?,
            );

        match self.scene.current  {
          Some(State::Start) => {
            let mut text = Text::new(r#"

Welcome to the snake game by eiberham

Ultimate Goal:

> Eat as much as you can and be
careful of not biting your own tail.

* Press the enter key to get started
* Press the escape key to quit
            "#);
            text.set_font("Arcade")
                .set_scale(24.0);

            canvas.draw(
                &text,
                DrawParam::from([ 32.0, 64.0 ]).color(Color::WHITE),
            );
          }
          Some(State::Running) => {
            // At this point the game has gotten started.
            // Changes the canvas background color and starts drawing all the actors in
            // the game along with the scorekeeping metrics.
            // To check for the rgb color go on https://www.colorspire.com/rgb-color-wheel/
            canvas = Canvas::from_frame(ctx, Color::from_rgb(11, 68, 42));
            self.snake.draw(&mut canvas, ctx);
            self.food.draw(&mut canvas, ctx);
            self.draw_score( &mut canvas, "level", Vec2::new(32.0, 32.0), self.level );
            self.draw_score( &mut canvas, "score", Vec2::new(720.0, 32.0), self.score );
          }
          Some(State::Over) => {
            let mut text = Text::new(r#"

Game Over Muahahaha !

Don't get discouraged, you'll
eventually get good at it.

* Press the enter key to continue
* Press the escape key to quit
            "#);
            text.set_font("Arcade")
                .set_scale(24.0);

            canvas.draw(
                &text,
                DrawParam::from([ 32.0, 160.0 ]).color(Color::RED),
            );
          }
          None => ()
        }

        canvas.finish(ctx)?;
        timer::sleep(std::time::Duration::from_millis(16));
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
        Some(keyboard::KeyCode::Return) => {
          if matches!(self.scene.current, Some(State::Over)) {
            self.scene.change(State::Start);
            self.restart();
          } else if matches!(self.scene.current, Some(State::Start)) {
            self.scene.change(State::Running);
          } else { return Ok(()) }

          // It looks like if I call the draw method from the outside it
          // throws the following error:
          // RenderError("starting Canvas outside of a frame")
          // So a workaround I found is to create a new frame beforehand
          // and end it afterwards
          // source: https://github.com/ggez/ggez/issues/1056
          _ctx.gfx.begin_frame()?; // .expect_err("couldn't begin the frame");
          self.draw(_ctx)?;
          _ctx.gfx.end_frame()?;
        }
        Some(keyboard::KeyCode::Escape) => {
          _ctx.request_quit();
        }
        _ => (),
      }
      Ok(())
    }
}