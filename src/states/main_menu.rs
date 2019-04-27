use amethyst::assets::Loader;
use amethyst::core::Transform;
use amethyst::ecs::Join;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection, VirtualKeyCode};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

use crate::components::MainMenuItem;
use crate::resources::{CurrentState, PlayerResource, ScoreResource, SpriteSheet};
use crate::states::GameplayState;
use crate::utility::load_sprite_sheet;

//use crate::components::PauseItem;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct MainMenuState;

impl SimpleState for MainMenuState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    world.add_resource(CurrentState::default());

    initialize_camera(world);
    initialize_menu_text(world);
  }

  fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
    if let StateEvent::Window(event) = &event {
      if is_key_down(&event, VirtualKeyCode::Space) {
        let mut game_state = _data.world.write_resource::<CurrentState>();
        game_state.resume();
        return Trans::Push(Box::new(GameplayState));
      }
    }
    Trans::None
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }

  fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    let world = _data.world;
    let main_menu_state_items = world.read_storage::<MainMenuItem>();
    let entities = world.entities();

    for (entity, _) in (&*entities, &main_menu_state_items).join() {
      entities
        .delete(entity)
        .expect("Unable to delete pause menu entitiy");
    }
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
fn initialize_menu_text(world: &mut World) {
  let font = world.read_resource::<Loader>().load(
    "textures/square.ttf",
    TtfFormat,
    Default::default(),
    (),
    &world.read_resource(),
  );

  let text_transform = UiTransform::new(
    "INCANTATION CATASTROPHE".to_string(),
    Anchor::Middle,
    0.,
    0.,
    1.,
    200.,
    50.,
    0,
  );

  world
    .create_entity()
    .with(text_transform)
    .with(UiText::new(
      font,
      "PAUSE".to_string(),
      [1., 1., 1., 1.],
      50.,
    ))
    .with(MainMenuItem)
    .build();
}
