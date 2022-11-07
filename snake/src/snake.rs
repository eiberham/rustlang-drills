use ggez::graphics::{ self, Rect, Canvas };
use std::collections::{ LinkedList };

use crate::place::*;
use crate::food::*;

#[derive(Copy, Clone)]
pub enum Direction {
  U,
  D,
  L,
  R
}

pub struct Size<T> {
  w: T,
  h: T
}

pub struct Snake {
  pub head: Rect,
  pub previous: Direction,
  pub current_direction: Option<Direction>,
  pub body: LinkedList<Rect>,
}

impl Snake {
  // associated function: struct impl fn that don't take self as parameter
  pub fn new() -> Snake {

    let size = Size {
      w: 0x20 as f32,
      h: 0x20 as f32
    };

    let place = Place::new(0x40 as f32, 0x40 as f32);
    let head = Rect::new(place.x, place.y, size.w, size.h);
    let mut body =  LinkedList::new();
    body.push_back(Rect::new(32.0, 64.0, size.w, size.h));

    Snake {
      head,
      previous: Direction::R,
      current_direction: Some(Direction::R),
      body
    }
  }

  pub fn draw(&mut self, canvas: &mut Canvas) -> () {

    for square in self.body.iter() {
      canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(*square)
                .color(graphics::Color::GREEN),
        );
    }

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head)
                .color(graphics::Color::YELLOW),
        );
  }

  // updates the snake's position based on its current direction
  pub fn update(&mut self, food: &mut Food) {
    if self.current_direction.is_some() {
      self.body.push_front(self.head);
      match self.current_direction.unwrap() {
        Direction::R => {
          self.head = Rect::new(self.head.x + 32.0, self.head.y, self.head.w, self.head.h);
          if self.head.x >= 960.0 {
            self.head.x = 0.0;
          }
        }
        Direction::L => {
          self.head = Rect::new(self.head.x - 32.0, self.head.y, self.head.w, self.head.h);
          if self.head.x <= -32.0 {
            self.head.x = 928.0;
          }
        }
        Direction::U => {
          self.head = Rect::new(self.head.x, self.head.y - 32.0, self.head.w, self.head.h);
          if self.head.y <= -32.0 {
            self.head.y = 928.0;
          }
        }
        Direction::D => {
          self.head = Rect::new(self.head.x, self.head.y + 32.0, self.head.w, self.head.h);
          if self.head.y >= 960.0 {
            self.head.y = -32.0;
          }
        }
      }
      if self.eats(&food) {
        food.fade();
      } else {
        self.body.pop_back();
      }
    }
  }

  pub fn eats(&mut self, food: &Food) -> bool {
    self.head.x == food.place.x &&
    self.head.y == food.place.y
  }

  pub fn divert(&mut self, direction: Direction) {
    self.current_direction = Some(direction);
    self.previous = direction;
  }
}