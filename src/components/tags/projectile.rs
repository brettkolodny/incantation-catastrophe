use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Projectile;

impl Component for Projectile {
  type Storage = NullStorage<Self>;
}