use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct PlayerProjectile;

impl Component for PlayerProjectile {
  type Storage = NullStorage<Self>;
}
