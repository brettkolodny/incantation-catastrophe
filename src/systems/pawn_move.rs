use amethyst::core::{nalgebra, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{CurrentDirection, Pawn, Player, Speed};

pub struct PawnMoveSystem;

impl<'s> System<'s> for PawnMoveSystem {
  type SystemData = (
    ReadStorage<'s, Player>,
    ReadStorage<'s, Pawn>,
    ReadStorage<'s, Speed>,
    WriteStorage<'s, CurrentDirection>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
  );

  fn run(
    &mut self,
    (players, pawns, speeds, mut directions, mut transforms, time): Self::SystemData,
  ) {
    if let Some((_, player_transform)) = (&players, &transforms).join().nth(0) {
      let player_transform = player_transform.clone();
      for (pawn_transform, pawn_speed, direction, _) in
        (&mut transforms, &speeds, &mut directions, &pawns).join()
      {
        let player_vector = player_transform.translation();
        let pawn_vector = pawn_transform.translation();

        let new_vector = player_vector - pawn_vector;
        let new_vector = nalgebra::base::Matrix::normalize(&new_vector);
        let new_vector = nalgebra::Unit::new_unchecked(new_vector);

        if new_vector.x.abs() > new_vector.y.abs() {
          if new_vector.x < 0. {
            direction.turn_right();
            pawn_transform.set_rotation_euler(0., 0., 4.71239);
          } else {
            direction.turn_left();
            pawn_transform.set_rotation_euler(0., 0., 1.5708);
          }
        } else {
          if new_vector.y < 0. {
            direction.turn_up();
            pawn_transform.set_rotation_euler(0., 0., 0.);
          } else {
            direction.turn_down();
            pawn_transform.set_rotation_euler(0., 0., 3.14159);
          }
        }

        pawn_transform.move_along_global(new_vector, time.delta_seconds() * pawn_speed.speed);
      }
    }
  }
}
