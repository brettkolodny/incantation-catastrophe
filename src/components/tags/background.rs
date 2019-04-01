use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Background;

impl Component for Background {
  type Storage = NullStorage<Self>;
}