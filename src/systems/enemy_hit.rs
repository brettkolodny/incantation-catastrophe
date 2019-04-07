use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{Enemy, PlayerProjectile, Projectile, Size};
use crate::utility::did_hit;

pub struct EnemyHitSystem;

impl<'s> System<'s> for EnemyHitSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Projectile>,
    ReadStorage<'s, PlayerProjectile>,
    ReadStorage<'s, Size>,
    WriteStorage<'s, Enemy>,
    Entities<'s>,
  );

  fn run(
    &mut self,
    (transforms, projectiles, player_projectiles, sizes, mut enemies, entities): Self::SystemData,
  ) {
    for (projectile_transform, _, _, projectile_size, projectile_entity) in (
      &transforms,
      &projectiles,
      &player_projectiles,
      &sizes,
      &entities,
    )
      .join()
    {
      for (enemy_transform, mut enemy, enemy_size, enemy_entity) in
        (&transforms, &mut enemies, &sizes, &entities).join()
      {
        if did_hit(
          (projectile_size, projectile_transform),
          (enemy_size, enemy_transform),
        ) {
          enemy.health -= 1;

          if enemy.health <= 0 {
            if let Err(e) = entities.delete(enemy_entity) {
              dbg!(e);
            }
          }

          if let Err(e) = entities.delete(projectile_entity) {
            dbg!(e);
          }
        }
      }
    }
  }
}
