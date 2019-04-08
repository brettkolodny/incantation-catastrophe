use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{CurrentDirection, Direction, Projectile, Speed};

pub struct ProjectileMoveSystem;

impl<'s> System<'s> for ProjectileMoveSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Projectile>,
    ReadStorage<'s, Speed>,
    ReadStorage<'s, CurrentDirection>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut transforms, projectiles, speeds, directions, time): Self::SystemData) {
    for (_, transform, speed, direction) in
      (&projectiles, &mut transforms, &speeds, &directions).join()
    {
      let projectile_loc = transform.translation();

      match direction.current_direction {
        Direction::Up => {
          transform.set_y(projectile_loc.y + (speed.speed * time.delta_seconds()));
        }
        Direction::Down => {
          transform.set_y(projectile_loc.y - (speed.speed * time.delta_seconds()));
        }
        Direction::Right => {
          transform.set_x(projectile_loc.x + (speed.speed * time.delta_seconds()));
        }
        Direction::Left => {
          transform.set_x(projectile_loc.x - (speed.speed * time.delta_seconds()));
        }
        Direction::Custom(vector) => {
          transform.move_along_global(vector, time.delta_seconds() * speed.speed);
        }
      }
    }
  }
}
