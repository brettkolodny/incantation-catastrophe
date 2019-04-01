use amethyst::ecs::{Component, NullStorage};

pub struct PlayerProjectile;

impl PlayerProjectile {
  pub fn new() -> Self {
    PlayerProjectile
  }
}

impl Component for PlayerProjectile {
  type Storage = NullStorage<Self>;
}

impl Default for PlayerProjectile {
  fn default() -> Self {
    PlayerProjectile
  }
}
