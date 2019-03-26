use amethyst::ecs::prelude::{Component, VecStorage};

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Default for Direction {
  fn default() -> Direction {
    Direction::Up
  }
}

pub struct CurrentDirection {
  current_direction: Direction,
}

impl CurrentDirection {
  pub fn new() -> CurrentDirection {
    CurrentDirection {
      current_direction: Direction::default(),
    }
  }

  pub fn turn_left(&mut self) {
    self.current_direction = Direction::Left;
  }

  pub fn turn_right(&mut self) {
    self.current_direction = Direction::Right;
  }

  pub fn turn_up(&mut self) {
    self.current_direction = Direction::Up;
  }

  pub fn turn_down(&mut self) {
    self.current_direction = Direction::Down;
  }
}

impl Component for CurrentDirection {
  type Storage = VecStorage<Self>;
}
