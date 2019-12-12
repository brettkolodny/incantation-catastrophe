mod background;
mod gameplay_item;
mod main_menu_item;
mod pause_item;
mod gameover_item;
mod title_item;

pub use self::{
    background::Background, gameplay_item::GameplayItem, main_menu_item::MainMenuItem,
    pause_item::PauseItem, gameover_item::GameoverItem, title_item::TitleItem
};

pub trait Tag {}
