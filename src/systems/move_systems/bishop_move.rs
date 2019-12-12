use amethyst::core::{
    math::{base::Matrix, Unit, Vector3},
    timing::Time,
    Transform,
};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use rand::Rng;

use crate::components::Bishop;
use crate::resources::CurrentState;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH, RADIUS};

pub struct BishopMoveSystem {
    pub move_timer: f32,
}

impl<'s> System<'s> for BishopMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Bishop>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(&mut self, (mut transforms, mut bishops, time, state): Self::SystemData) {
        if !state.is_gameplay() {
            return;
        }
        for (mut bishop, transform) in (&mut bishops, &mut transforms).join() {
            if bishop.time_since_move >= self.move_timer {
                let angle = rand::thread_rng().gen_range(0, 360) as f32;

                let circle_vector = {
                    let x = RADIUS * angle.sin() + GAMEPLAY_AREA_WIDTH / 2.;
                    let y = RADIUS * angle.cos() + GAMEPLAY_AREA_HEIGHT / -2.;
                    let z = 0.;

                    let circle_vector = Vector3::new(x, y, z);
                    circle_vector
                };

                transform.set_translation_xyz(circle_vector.x, circle_vector.y, circle_vector.z);

                let center_vector =
                    //Vector3::new(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., 0.);
                    Vector3::new(RADIUS, RADIUS, 0.);

                let move_vector = Matrix::normalize(&(center_vector - circle_vector));

                let distance = rand::thread_rng().gen_range(0, RADIUS as i32) as f32;

                transform.prepend_translation_along(Unit::new_unchecked(move_vector), distance);
                bishop.time_since_move = 0.;
            } else {
                bishop.time_since_move += time.delta_seconds();
            }
        }
    }
}
