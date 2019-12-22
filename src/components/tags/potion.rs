use amethyst::ecs::{Component, NullStorage};

use crate::components::Tag;

#[derive(Default)]
pub struct Potion;

impl Component for Potion {
    type Storage = NullStorage<Self>;
}

impl Tag for Potion {}
