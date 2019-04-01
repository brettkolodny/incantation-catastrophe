// use amethyst::ecs::{Component, DenseVecStorage, Join};
// use amethyst::assets::{AssetStorage, Loader};
// use amethyst::renderer::{
//   PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
// };
// use amethyst::prelude::*;

// use crate::components::{Direction, Player};

// pub struct PlayerProjectile;

// impl Component for PlayerProjectile {
//   type Storage = DenseVecStorage<Self>;
// }

// pub fn initialize_shot(mut world: World) {
//   let projectile_direction: Option<CurrentDirection> = {
//     let player_storage = world.read_storage::<Player>();
//     let direction_storage = world.read_storage::<CurrentDirection>();
//     let mut set_direction: Option<CurrentDirection> = None;
//     for (_, direction) in (&player_storage, &direction_storage).join() {
//       let mut new_direction = CurrentDirection::new();
//       match direction.current_direction {
//         Direction::Up => new_direction.turn_up(),
//         Direction::Down => new_direction.turn_down(),
//         Direction::Left => new_direction.turn_left(),
//         Direction::Right => new_direction.turn_right(),
//       }

//       set_direction = Some(new_direction);
//     }

//     set_direction
//   };
// }
