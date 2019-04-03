use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Projectile {
  pub width: f32,
  pub height: f32,
}

impl Projectile {
  pub fn new(width: f32, height: f32) -> Self {
    Projectile { width, height }
  }
}

impl Component for Projectile {
  type Storage = DenseVecStorage<Self>;
}
