use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage, VecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
  PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const GAMEPLAY_AREA_WIDTH: f32 = 1080.;
pub const GAMEPLAY_AREA_HEIGHT: f32 = 720.;

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

#[derive(Default)]
pub struct GameplayItem;

impl Component for GameplayItem {
  type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Player {
  width: f32,
  height: f32,
}

impl Player {
  pub fn new() -> Player {
    Player {
      width: 10.,
      height: 10.,
    }
  }
}

impl Component for Player {
  type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Speed {
  pub speed: f32,
}

impl Speed {
  pub fn new(speed: f32) -> Speed {
    Speed { speed }
  }
}

impl Component for Speed {
  type Storage = VecStorage<Self>;
}
