use amethyst::ecs::{Component, NullStorage};

use crate::components::Tag;

#[derive(Default)]
pub struct TitleItem;

impl Component for TitleItem {
    type Storage = NullStorage<Self>;
}

impl Tag for TitleItem {}
