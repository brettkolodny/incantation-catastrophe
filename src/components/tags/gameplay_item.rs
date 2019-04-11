use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct GameplayItem;

impl Component for GameplayItem {
    type Storage = NullStorage<Self>;
}
