use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Size {
  pub width: f32,
  pub height: f32,
}

impl Size {
  pub fn new(width: f32, height: f32) -> Self {
    Size { width, height }
  }
}

impl Component for Size {
  type Storage = DenseVecStorage<Self>;
}
