mod direction;
mod enemy;
mod health;
mod player;
mod projectile;
mod size;
mod speed;
mod tags;

pub use self::{
    direction::{CurrentDirection, Direction},
    enemy::Enemy,
    player::Player,
    projectile::Projectile,
    size::Size,
    speed::Speed,
    health::Health,
    tags::{Background, Bishop, GameplayItem, Pawn, PlayerProjectile},
};

pub trait Dimensions {
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
}
