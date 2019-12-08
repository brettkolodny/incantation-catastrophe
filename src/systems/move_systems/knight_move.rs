use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{CurrentDirection, Direction, Knight, Speed};
use crate::resources::CurrentState;

pub struct KnightMoveSystem;

impl<'s> System<'s> for KnightMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Knight>,
        ReadStorage<'s, Speed>,
        ReadStorage<'s, CurrentDirection>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (mut transforms, knights, speeds, directions, time, state): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        for (_, transform, speed, direction) in
            (&knights, &mut transforms, &speeds, &directions).join()
        {
            let knight_loc_x = transform.translation().x;
            let knight_loc_y = transform.translation().y;

            match direction.current_direction {
                Direction::Up => {
                    transform
                        .set_translation_y(knight_loc_y + (speed.speed * time.delta_seconds()));
                }
                Direction::Down => {
                    transform
                        .set_translation_y(knight_loc_y - (speed.speed * time.delta_seconds()));
                }
                Direction::Right => {
                    transform
                        .set_translation_x(knight_loc_x + (speed.speed * time.delta_seconds()));
                }
                Direction::Left => {
                    transform
                        .set_translation_x(knight_loc_x - (speed.speed * time.delta_seconds()));
                }
                Direction::Custom(vector) => {
                    transform.prepend_translation_along(vector, time.delta_seconds() * speed.speed);
                }
            }
        }
    }
}
