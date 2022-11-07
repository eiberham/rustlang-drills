use rand::{ self };
use rand::seq::SliceRandom;

// #[derive(Eq, PartialEq)]
pub struct Place {
  pub x: f32,
  pub y: f32
}

impl Place {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  pub fn get() -> Self {
    let board: Vec<u32> = (0..960)
      .step_by(32)
      .map(|v| v)
      .collect();

    let randomize = move || *board.to_vec()
        .choose(&mut rand::thread_rng())
        .unwrap();

    let x = randomize();
    let y = randomize();

    Self {
      x: x as f32,
      y: y as f32
    }
  }
}
