use amethyst::ecs::prelude::{Component, VecStorage};

pub struct Speed {
  pub speed: f32,
}

impl Speed {
  pub fn new(speed: f32) -> Speed {
    Speed { speed }
  }
}

impl Default for Speed {
  fn default() -> Speed {
    Speed { speed: 1000. }
  }
}

impl Component for Speed {
  type Storage = VecStorage<Self>;
}
