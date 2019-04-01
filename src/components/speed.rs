use amethyst::ecs::prelude::{Component, VecStorage};

#[derive(Default)]
pub struct Speed {
  pub speed: f32,
}

impl Speed {
  pub fn new(speed: f32) -> Speed {
    Speed { speed }
  }
}

impl Component for Speed {
  type Storage = VecStorage<Self>;
}