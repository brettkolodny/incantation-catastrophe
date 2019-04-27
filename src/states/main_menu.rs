use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};

//use crate::components::PauseItem;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct MainMenuState;

impl SimpleState for MainMenuState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

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
// fn initialize_pause_text(world: &mut World) {
//   let font = world.read_resource::<Loader>().load(
//     "font/square.ttf",
//     TtfFormat,
//     Default::default(),
//     (),
//     &world.read_resource(),
//   );

//   let text_transform = UiTransform::new(
//     "PAUSE".to_string(),
//     Anchor::Middle,
//     0.,
//     0.,
//     1.,
//     200.,
//     50.,
//     0,
//   );

//   world
//     .create_entity()
//     .with(text_transform)
//     .with(UiText::new(
//       font,
//       "PAUSE".to_string(),
//       [1., 1., 1., 1.],
//       50.,
//     ))
//     .with(PauseItem)
//     .build();
// }
