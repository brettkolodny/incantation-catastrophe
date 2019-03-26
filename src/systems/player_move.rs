use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::incantation_catastrophe::{
  current_direction::CurrentDirection, player::Player, speed::Speed,
};

pub struct PlayerMoveSystem;

impl<'s> System<'s> for PlayerMoveSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Speed>,
    WriteStorage<'s, CurrentDirection>,
    ReadStorage<'s, Player>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, speeds, mut directions, players, input): Self::SystemData) {
    for (transform, direction, speed, _) in
      (&mut transforms, &mut directions, &speeds, &players).join()
    {
      let horizontal_movement = input.axis_value("horizontal");
      let vertical_movement = input.axis_value("vertical");

      let mut change_x: f32 = {
        if let Some(amount) = horizontal_movement {
          speed.speed * amount as f32
        } else {
          0.
        }
      };

      let mut change_y: f32 = {
        if let Some(amount) = vertical_movement {
          speed.speed * amount as f32
        } else {
          0.
        }
      };

      if change_x > 0. {
        transform.set_rotation_euler(0., 0., 1.5708);
        direction.turn_right();
      } else if change_x < 0. {
        transform.set_rotation_euler(0., 0., 4.71239);
        direction.turn_left();
      }

      if change_y > 0. {
        transform.set_rotation_euler(0., 0., 0.);
        direction.turn_up();
      } else if change_y < 0. {
        transform.set_rotation_euler(0., 0., 3.14159);
        direction.turn_down();
      }

      if change_y != 0. && change_x != 0. {
        let result_vector: f32 = speed.speed;
        let temp = result_vector / (2.0 as f32).sqrt();
        let mut new_x = temp;
        let mut new_y = temp;
        if change_x < 0. {
          new_x = new_x * -1.;
        }
        if change_y < 0. {
          new_y = new_y * -1.;
        }

        change_x = new_x;
        change_y = new_y;
      }

      transform.set_x(transform.translation().x + change_x);
      transform.set_y(transform.translation().y + change_y);
    }
  }
}
