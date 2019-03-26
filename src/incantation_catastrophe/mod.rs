pub mod current_direction;
pub mod player;
pub mod speed;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::prelude::{Component, NullStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
  PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const GAMEPLAY_AREA_WIDTH: f32 = 1920.;
pub const GAMEPLAY_AREA_HEIGHT: f32 = 1080.;

#[derive(Default)]
pub struct GameplayItem;

impl Component for GameplayItem {
  type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Background;

impl Component for Background {
  type Storage = NullStorage<Self>;
}

pub fn load_sprite_sheet(
  _world: &mut World,
  _sprite_sheet: &str,
  _sprite_sheet_ron: &str,
) -> SpriteSheetHandle {
  let texture_handle = {
    let loader = _world.read_resource::<Loader>();
    let texture_storage = _world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      _sprite_sheet,
      PngFormat,
      TextureMetadata::srgb_scale(),
      (),
      &texture_storage,
    )
  };

  let loader = _world.read_resource::<Loader>();
  let sprite_sheet_store = _world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    _sprite_sheet_ron,
    SpriteSheetFormat,
    texture_handle,
    (),
    &sprite_sheet_store,
  )
}
