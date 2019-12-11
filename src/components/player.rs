use amethyst::core::{math::Vector3, transform::Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{sprite::SpriteSheetHandle, SpriteRender};

use crate::components::{tags::GameplayItem, CurrentDirection, Health, Size, Speed};
use crate::resources::PlayerResource;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

#[derive(Default)]
pub struct Player {
    pub time_since_shot: f32,
    pub cooldown: f32,
    pub player_immune: bool,
    pub player_immune_time: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            time_since_shot: 3.,
            cooldown: 0.25,
            player_immune: false,
            player_immune_time: 3.,
        }
    }

    pub fn initialize(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(
            GAMEPLAY_AREA_WIDTH / 2.,
            GAMEPLAY_AREA_HEIGHT / -2.,
            0.,
        );
        local_transform.set_scale(Vector3::new(2.5, 2.5, 1.));

        let sprite_render = {
            SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: 4,
            }
        };

        let player = Some(
            world
                .create_entity()
                .with(sprite_render)
                .with(local_transform)
                .with(Player::new())
                .with(Speed::new(5.))
                .with(Size::new(50., 50.))
                .with(CurrentDirection::default())
                .with(GameplayItem)
                .with(Health::default())
                .build(),
        );

        world.insert(PlayerResource { player });
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
