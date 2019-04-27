use amethyst::ecs::{Read, WriteStorage, System};
use amethyst::renderer::SpriteRender;

use crate::resources::PlayerResource;

pub struct PlayerImmuneSystem;

impl<'s> System<'s> for PlayerImmuneSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        Read<'s, PlayerResource>,
    );

    fn run(&mut self, (sprite_renders, player): Self::SystemData) {

    }
}