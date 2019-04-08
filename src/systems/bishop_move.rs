use amethyst::core::{
  nalgebra::{base::Matrix, Unit, Vector3},
  timing::Time,
  Transform,
};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use rand::Rng;

use crate::components::Bishop;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct BishopMoveSystem {
  pub move_timer: f32,
}

impl<'s> System<'s> for BishopMoveSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Bishop>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut transforms, mut bishops, time): Self::SystemData) {
    for (mut bishop, transform) in (&mut bishops, &mut transforms).join() {
      if bishop.time_since_move >= self.move_timer {
        let radius = GAMEPLAY_AREA_HEIGHT / 2.;
        let angle = rand::thread_rng().gen_range(0, 360) as f32;

        let circle_vector = {
          let x = radius * angle.sin() + GAMEPLAY_AREA_WIDTH / 2.;
          let y = radius * angle.cos() + GAMEPLAY_AREA_HEIGHT / 2.;
          let z = 0.;

          let circle_vector = Vector3::new(x, y, z);
          circle_vector
        };

        transform.set_xyz(circle_vector.x, circle_vector.y, circle_vector.z);

        let center_vector = Vector3::new(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., 0.);

        let move_vector = Matrix::normalize(&(center_vector - circle_vector));

        let distance = rand::thread_rng().gen_range(0, GAMEPLAY_AREA_HEIGHT as i32) as f32;

        transform.move_along_global(Unit::new_unchecked(move_vector), distance);
        bishop.time_since_move = 0.;
      } else {
        bishop.time_since_move += time.delta_seconds();
      }
    }
  }
}