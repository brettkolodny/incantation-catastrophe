mod attribute;
mod enemy;
mod player;
mod projectile;
mod tags;

pub use self::{
    player::Player,
    projectile::{Projectile, PlayerProjectile},
    tags::{Background, GameplayItem},
    enemy::{Pawn, Bishop, Enemy},
    attribute::*,
};

pub trait Dimensions {
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
}
