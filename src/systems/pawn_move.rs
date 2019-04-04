use amethyst::core::{nalgebra, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{Pawn, Player, Speed};

pub struct PawnMoveSystem;

impl<'s> System<'s> for PawnMoveSystem {
  type SystemData = (
    ReadStorage<'s, Player>,
    ReadStorage<'s, Pawn>,
    ReadStorage<'s, Speed>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
  );

  fn run(&mut self, (players, pawns, speeds, mut transforms, time): Self::SystemData) {
    let player_transform = (&players, &transforms).join().nth(0).unwrap().1.clone();

    for (pawn_transform, pawn_speed, _) in (&mut transforms, &speeds, &pawns).join() {
      let player_vector = player_transform.translation();
      let pawn_vector = pawn_transform.translation();

      let new_vector = player_vector - pawn_vector;
      let new_vector = nalgebra::base::Matrix::normalize(&new_vector);
      let new_vector = nalgebra::Unit::new_unchecked(new_vector);

      pawn_transform.move_along_global(new_vector, time.delta_seconds() * pawn_speed.speed);
    }
  }
}
