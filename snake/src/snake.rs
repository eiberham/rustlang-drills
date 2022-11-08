use ggez::graphics::{ self, Canvas };
use ggez::audio::{ Source, SoundSource };
use ggez::Context;
use std::collections::{ LinkedList };

use crate::food::*;
use crate::tile::*;

#[derive(Copy, Clone)]
pub enum Direction {
  U,
  D,
  L,
  R
}

pub struct Snake {
  pub head: Tile,
  pub previous: Direction,
  pub current_direction: Option<Direction>,
  pub body: LinkedList<Tile>,
  // pub speed: f32
}

impl Snake {
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

  // updates the snake's position based on its current direction
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

  pub fn eats(&mut self, food: &Food) -> bool {
    self.head == food.piece
  }

  pub fn divert(&mut self, direction: Direction) {
    self.current_direction = Some(direction);
    self.previous = direction;
  }

  pub fn collides(&mut self) -> bool {
    self.body.iter().any(|&x| x == self.head )
  }
}