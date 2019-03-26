use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{ SpriteRender, SpriteSheetHandle };

use crate::incantation_catastrophe::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH, speed::Speed, GameplayItem, current_direction::CurrentDirection};

#[derive(Default)]
pub struct Player {
  pub width: f32,
  pub height: f32,
}

impl Player {
  pub fn new() -> Player {
    Player {
      width: 1.,
      height: 200.,
    }
  }
}

impl Component for Player {
  type Storage = DenseVecStorage<Self>;
}

pub fn initialize_player(_world: &mut World, _sprite_sheet_handle: SpriteSheetHandle) {
  let mut local_transform = Transform::default();
  local_transform.set_xyz(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., 0.);
  local_transform.set_scale(10., 10., 1.);

  let sprite_render = {
    SpriteRender {
      sprite_sheet: _sprite_sheet_handle,
      sprite_number: 0,
    }
  };

  _world
    .create_entity()
    .with(sprite_render)
    .with(local_transform)
    .with(Player::new())
    .with(Speed::new(5.))
    .with(CurrentDirection::new())
    .with(GameplayItem)
    .build();
}