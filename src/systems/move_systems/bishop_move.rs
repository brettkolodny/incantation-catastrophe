use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use rand::Rng;
use std::f32::consts::PI;

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
                let angle = (rand::thread_rng().gen_range(0, 99) as f32 / 100.) * 2. * PI;
                let r = RADIUS * (rand::thread_rng().gen_range(0, 99) as f32 / 100.).sqrt();

                let x = r * angle.cos() + GAMEPLAY_AREA_WIDTH / 2.;
                let y = r * angle.sin() - GAMEPLAY_AREA_HEIGHT / 2.;

                transform.set_translation_xyz(x, y, 0.);

                bishop.time_since_move = 0.;
            } else {
                bishop.time_since_move += time.delta_seconds();
            }
        }
    }
}
