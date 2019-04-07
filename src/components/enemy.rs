use amethyst::ecs::{Component, DenseVecStorage};

pub struct Enemy {
  pub health: i32,
}

impl Enemy {
  pub fn pawn() -> Self {
    Enemy { health: 1 }
  }

  pub fn bishop() -> Self {
    Enemy { health: 3 }
  }
}

impl Component for Enemy {
  type Storage = DenseVecStorage<Self>;
}
