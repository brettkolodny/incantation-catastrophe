use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Bishop {
  pub time_since_move: f32,
}

impl Component for Bishop {
  type Storage = DenseVecStorage<Self>;
}
