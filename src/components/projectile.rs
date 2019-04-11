use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Projectile;

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
