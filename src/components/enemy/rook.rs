use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Rook;

impl Component for Rook {
    type Storage = NullStorage<Self>;
}
