use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Knight;

impl Component for Knight {
    type Storage = NullStorage<Self>;
}
