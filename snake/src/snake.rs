//! Snake abstraction.
//!
//! Provides an abstraction over a snake. Will be in charge of eating
//! food and roaming around the entire playground.
//!

use ggez::graphics::{ self, Canvas };
use ggez::audio::{ Source, SoundSource };
use ggez::Context;
use std::collections::{ LinkedList };

use crate::food::*;
use crate::tile::*;

/// Represents the direction the snake could potentially move to
#[derive(Copy, Clone)]
pub enum Direction {
  /// Up
  U,
  /// Down
  D,
  /// Left
  L,
  /// Right
  /// This will be the default direction when starting the game
  R
}


/// Represents the game's snake.
///
/// The head will be a tile and will help to move around the playground,
/// identify collisions, food consumption, etc.
///
/// The previous field holds the previous direction the snake was heading
/// towards.
///
/// The current direction is held in an <Option> because we would want
/// to stop the snake when colliding with itself by assigning a None.
///
/// The body is a linked list holding tiles. It is helpful when it comes
/// to adding new tiles on top of the snake.
pub struct Snake {
  pub head: Tile,
  pub previous: Direction,
  pub current_direction: Option<Direction>,
  pub body: LinkedList<Tile>,
  // pub speed: f32
}

impl Snake {
  /// Construct a snake representing the main character of the game.
  pub fn new() -> Snake {

    let head = Tile::new(64.0, 64.0);
    let tile = Tile::new(32.0, 64.0);

    let mut body =  LinkedList::new();
    body.push_back(tile);

    Snake {
      head,
      previous: Direction::R,
      current_direction: Some(Direction::R),
      body
    }
  }

  /// Draws the snake on the canvas.
  pub fn draw(&mut self, canvas: &mut Canvas) -> () {

    for square in self.body.iter() {
      let mut tile = square.clone();
      canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(tile.draw())
                .color(graphics::Color::GREEN),
        );
    }

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head.draw())
                .color(graphics::Color::YELLOW),
        );
  }

  /// Updates the snake's position based on its current direction.
  ///
  /// Handles the movement logic around the playground and ensures
  /// that whenever the snake surpasses the edge border it will
  /// continue its course
  pub fn update(&mut self, food: &mut Food, ctx: &mut Context) {
    if self.current_direction.is_some() {
      self.body.push_front(self.head);
      match self.current_direction.unwrap() {
        Direction::R => {
          self.head.move_x(32.0);
          if self.head.x >= 960.0 { self.head.x = 0.0; }
        }
        Direction::L => {
          self.head.move_x(-32.0);
          if self.head.x <= -32.0 { self.head.x = 928.0; }
        }
        Direction::U => {
          self.head.move_y(-32.0);
          if self.head.y <= -32.0 {
            self.head.y = 928.0;
          }
        }
        Direction::D => {
          self.head.move_y(32.0);
          if self.head.y >= 960.0 {
            self.head.y = -32.0;
          }
        }
      }
      if self.eats(&food){
        let mut sound = Source::new(ctx, "/sound.wav").unwrap();
        sound.play_detached(ctx).unwrap();
        food.serve();
      } else {
        self.body.pop_back();
      }
    }
  }

  /// Indicates if the snake ate or not by comparing its head's position
  /// with that of the food.
  ///
  /// Since both the head and the food are the same type and the
  /// partialeq trait was implemented comparing one against the other is
  /// utterly possible.
  fn eats(&mut self, food: &Food) -> bool {
    self.head == food.piece
  }

  /// Changes the snake's direction.
  ///
  /// It's simply setting the direction-related attributes fo the snake.
  /// For the time being an option is being used on the current_direction
  /// attribute. That is because we'd want to set it to none once the
  /// snake collides with itself.
  pub fn change_direction(&mut self, direction: Direction) {
    self.current_direction = Some(direction);
    self.previous = direction;
  }

  /// Returns a boolean indicating whether the snake collided or not.
  ///
  /// Since the snake's body is a linked list of tiles, the partialeq
  /// trait implementation is being taken advantage of in order to
  /// compare the body position with that of the head.
  pub fn collides(&mut self) -> bool {
    self.body.iter().any(|&x| x == self.head )
  }
}