use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Pawn;

impl Component for Pawn {
    type Storage = NullStorage<Self>;
}
