use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::incantation_catastrophe::{Player, Speed};

pub struct PlayerMoveSystem;

impl<'s> System<'s> for PlayerMoveSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Speed>,
    ReadStorage<'s, Player>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, speeds, players, input): Self::SystemData) {
    for (transform, speed, _) in (&mut transforms, &speeds, &players).join() {
      let horizontal_movement = input.axis_value("horizontal");
      let vertical_movement = input.axis_value("vertical");

      let x_change: i32 = {
        if let Some(amount) = vertical_movement {
          if amount > 0. {
            transform.set_rotation_euler(0., 0., 0.);
          } else if amount < 0. {
            transform.set_rotation_euler(0., 0., 3.14159);
          }
          let player_y = transform.translation().y;
          let scaled_amount = speed.speed * amount as f32;
          transform.set_y(player_y + scaled_amount);
        }
        14
      };

      if let Some(amount) = horizontal_movement {
        if amount > 0. {
          transform.set_rotation_euler(0., 0., 1.5708);
        } else if amount < 0. {
          transform.set_rotation_euler(0., 0., 4.71239);
        }
        let player_x = transform.translation().x;
        let scaled_amount = speed.speed * amount as f32;
        transform.set_x(player_x + scaled_amount);
      }
    }
  }
}
