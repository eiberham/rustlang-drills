//! Scene abstraction.
//!
//! Handles the game's scenes i.e menu scene
//! transition to game scene
//!


// Indicates the current state of the game. These
// states will indicate which scene to display for
// the user to make a choice
#[derive(Copy, Clone, smart_default::SmartDefault )]
pub enum State {
  // The game hasn't started yet
  // It is awaiting the user to hit enter to start
  #[default]
  Start,
  // The game has already started.
  Running
}

#[derive(smart_default::SmartDefault)]
pub struct Scene {
  #[default(Some(State::default()))]
  pub current: Option<State>
}

impl Scene {
  pub fn new() -> Self {  Self::default() }

  /// Changes from one scene to another.
  pub fn change(&mut self, state: State) -> () {
    self.current = Some(state)
  }
}