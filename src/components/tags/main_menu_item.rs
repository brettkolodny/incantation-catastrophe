use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct MainMenuItem;

impl Component for MainMenuItem {
    type Storage = NullStorage<Self>;
}
