use crate::components::Tag;
use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct GameoverItem;

impl Component for GameoverItem {
    type Storage = NullStorage<Self>;
}

impl Tag for GameoverItem {}
