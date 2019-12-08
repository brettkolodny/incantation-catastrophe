use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{CurrentDirection, Direction, Projectile, Speed};
use crate::resources::CurrentState;

pub struct ProjectileMoveSystem;

impl<'s> System<'s> for ProjectileMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, Speed>,
        ReadStorage<'s, CurrentDirection>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (mut transforms, projectiles, speeds, directions, time, state): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        for (_, transform, speed, direction) in
            (&projectiles, &mut transforms, &speeds, &directions).join()
        {

            let projectile_loc_x = transform.translation().x;
            let projectile_loc_y = transform.translation().y;

            match direction.current_direction {
                Direction::Up => {
                    transform
                        .set_translation_y(projectile_loc_y + (speed.speed * time.delta_seconds()));
                }
                Direction::Down => {
                    transform
                        .set_translation_y(projectile_loc_y - (speed.speed * time.delta_seconds()));
                }
                Direction::Right => {
                    transform
                        .set_translation_x(projectile_loc_x + (speed.speed * time.delta_seconds()));
                }
                Direction::Left => {
                    transform
                        .set_translation_x(projectile_loc_x - (speed.speed * time.delta_seconds()));
                }
                Direction::Custom(vector) => {
                    transform.prepend_translation_along(vector, time.delta_seconds() * speed.speed);
                }
            }
        }
    }
}
