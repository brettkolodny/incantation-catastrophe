mod background;
mod gameover_item;
mod gameplay_item;
mod main_menu_item;
mod pause_item;
mod title_item;

pub use self::{
    background::Background, gameover_item::GameoverItem, gameplay_item::GameplayItem,
    main_menu_item::MainMenuItem, pause_item::PauseItem, title_item::TitleItem,
};

pub trait Tag {}
