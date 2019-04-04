use amethyst::ecs::{Component, DenseVecStorage};

pub struct Enemy {
  pub height: f32,
  pub width: f32,
  pub health: i32,
}

impl Enemy {
  pub fn pawn() -> Self {
    Enemy {
      height: 48.,
      width: 48.,
      health: 1,
    }
  }
}

impl Component for Enemy {
  type Storage = DenseVecStorage<Self>;
}
