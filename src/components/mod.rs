mod attribute;
mod enemy;
mod player;
mod projectile;
mod tags;

pub use self::{
    attribute::*,
    enemy::{Bishop, Enemy, Pawn, Rook},
    player::Player,
    projectile::{PlayerProjectile, Projectile},
    tags::{Background, GameplayItem},
};

pub trait Dimensions {
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
}
