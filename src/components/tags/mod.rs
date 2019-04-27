mod background;
mod gameplay_item;
mod pause_item;

pub use self::{
    background::Background, gameplay_item::GameplayItem, pause_item::PauseItem,
};

pub trait Tag {

}