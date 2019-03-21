use crate::incantation_catastrophe::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity, Join, NullStorage};
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::{
  Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
  SpriteSheetHandle, Texture, TextureMetadata, VirtualKeyCode,
};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

pub struct GameplayState;

impl SimpleState for GameplayState {
  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    let world = _data.world;
    world.register::<Camera>();
    world.register::<Transform>();

    initialize_camera(world);
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }
}

fn initialize_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_z(1.0);
  world
    .create_entity()
    .with(Camera::from(Projection::orthographic(
      0.0,
      GAMEPLAY_AREA_WIDTH,
      0.0,
      GAMEPLAY_AREA_HEIGHT,
    )))
    .with(transform)
    .build();
}
