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

      if let Some(amount) = vertical_movement {
        let player_y = transform.translation().y;
        let scaled_amount = speed.speed * amount as f32;
        transform.set_y(player_y + scaled_amount);
      }

      if let Some(amount) = horizontal_movement {
        let player_x = transform.translation().x;
        let scaled_amount = speed.speed * amount as f32;
        transform.set_x(player_x + scaled_amount);
      }
    }
  }
}
