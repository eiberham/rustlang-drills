// Indicates the current state of the game. These states
// will indicate which scene to display for the user to
// make a choice
#[derive(Copy, Clone, smart_default::SmartDefault )]
pub enum State {
  // The game hasn't started yet
  // It is awaiting the user to hit esc to start
  #[default]
  Start,
  // The game has already started
  Running,
  // The user has hit the `p` key
  Pause
}

#[derive(smart_default::SmartDefault)]
pub struct Scene {
  #[default(Some(State::default()))]
  pub current: Option<State>
}

impl Scene {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn change(&mut self, state: State) -> () {
    self.current = Some(state)
  }
}