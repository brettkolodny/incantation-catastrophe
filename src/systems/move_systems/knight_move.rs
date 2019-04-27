use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{CurrentDirection, Direction, Knight, Speed};

pub struct KnightMoveSystem;

impl<'s> System<'s> for KnightMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Knight>,
        ReadStorage<'s, Speed>,
        ReadStorage<'s, CurrentDirection>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, knights, speeds, directions, time): Self::SystemData) {
        for (_, transform, speed, direction) in
            (&knights, &mut transforms, &speeds, &directions).join()
        {
            let knight_loc = transform.translation();

            match direction.current_direction {
                Direction::Up => {
                    transform.set_y(knight_loc.y + (speed.speed * time.delta_seconds()));
                }
                Direction::Down => {
                    transform.set_y(knight_loc.y - (speed.speed * time.delta_seconds()));
                }
                Direction::Right => {
                    transform.set_x(knight_loc.x + (speed.speed * time.delta_seconds()));
                }
                Direction::Left => {
                    transform.set_x(knight_loc.x - (speed.speed * time.delta_seconds()));
                }
                Direction::Custom(vector) => {
                    transform.move_along_global(vector, time.delta_seconds() * speed.speed);
                }
            }
        }
    }
}
