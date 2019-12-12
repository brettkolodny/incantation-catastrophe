mod background;
mod gameplay_item;
mod main_menu_item;
mod pause_item;
mod gameover_item;

pub use self::{
    background::Background, gameplay_item::GameplayItem, main_menu_item::MainMenuItem,
    pause_item::PauseItem, gameover_item::GameoverItem
};

pub trait Tag {}
