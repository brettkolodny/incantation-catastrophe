use amethyst::ecs::{Component, NullStorage};

use crate::components::Tag;

#[derive(Default)]
pub struct PauseItem;

impl Component for PauseItem {
    type Storage = NullStorage<Self>;
}

impl Tag for PauseItem {}
