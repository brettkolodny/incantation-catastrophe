use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{
  PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const GAMEPLAY_AREA_WIDTH: f32 = 1920.;
pub const GAMEPLAY_AREA_HEIGHT: f32 = 1080.;

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
