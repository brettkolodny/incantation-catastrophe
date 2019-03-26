use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection, SpriteRender, SpriteSheetHandle};

use crate::incantation_catastrophe::player::initialize_player;

use crate::incantation_catastrophe::{
  load_sprite_sheet, Background, GameplayItem, GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH,
};

pub struct GameplayState;

impl SimpleState for GameplayState {
  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    let world = _data.world;
    world.register::<Camera>();
    world.register::<Transform>();
    world.register::<GameplayItem>();
    world.register::<SpriteRender>();
    world.register::<SpriteSheetHandle>();

    let sprite_sheet_handle = load_sprite_sheet(
      world,
      "textures/pong_spritesheet.png",
      "textures/pong_spritesheet.ron",
    );

    let arena_sprite_sheet_handle = load_sprite_sheet(
      world,
      "textures/arena_sprite.png",
      "textures/arena_sprite.ron",
    );

    initialize_arena(world, arena_sprite_sheet_handle);
    initialize_player(world, sprite_sheet_handle);
    initialize_camera(world);
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }
}

fn initialize_camera(_world: &mut World) {
  let mut transform = Transform::default();
  transform.set_z(1.0);
  _world
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

pub fn initialize_arena(_world: &mut World, _sprite_sheet_handle: SpriteSheetHandle) {
  let mut local_transform = Transform::default();
  local_transform.set_xyz(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., 0.);

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
    .with(GameplayItem)
    .with(Background)
    .build();
}
