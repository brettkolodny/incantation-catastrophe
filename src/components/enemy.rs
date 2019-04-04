use amethyst::ecs::{Component, DenseVecStorage};

pub struct Enemy {
  pub height: f32,
  pub width: f32,
  pub health: u32,
}

impl Enemy {
  pub fn pawn() -> Self {
    Enemy {
      height: 64.,
      width: 64.,
      health: 1,
    }
  }
}

impl Component for Enemy {
  type Storage = DenseVecStorage<Self>;
}
