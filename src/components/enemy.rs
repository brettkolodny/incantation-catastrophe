use amethyst::ecs::{Component, DenseVecStorage};

pub struct Enemy {
  pub height: f32,
  pub width: f32,
  pub health: u32,
}

impl Enemy {
  fn new(height: f32, width: f32, health: u32) -> Self {
    Enemy {
      height,
      width,
      health,
    }
  }

  pub fn pawn() -> Self {
    Enemy {
      height: 10.,
      width: 10.,
      health: 1,
    }
  }
}

impl Component for Enemy {
  type Storage = DenseVecStorage<Self>;
}
