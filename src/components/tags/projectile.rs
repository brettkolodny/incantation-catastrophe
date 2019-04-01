use amethyst::ecs::{Component, NullStorage}

pub struct Projectile{};

impl Component for Projectile {
  type Storage = NullStorage<Self>;
}