use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{Enemy, PlayerProjectile, Projectile};

pub struct EnemyHitSystem;

impl<'s> System<'s> for EnemyHitSystem {
  type SystemData = (
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Projectile>,
    ReadStorage<'s, PlayerProjectile>,
    WriteStorage<'s, Enemy>,
    Entities<'s>,
  );

  fn run(
    &mut self,
    (transforms, projectiles, player_projectiles, mut enemies, entities): Self::SystemData,
  ) {
    for (projectile_transform, projectile, _, projectile_entity) in
      (&transforms, &projectiles, &player_projectiles, &entities).join()
    {
      for (enemy_transform, mut enemy, enemy_entity) in
        (&transforms, &mut enemies, &entities).join()
      {
        if did_hit((projectile, projectile_transform), (enemy, enemy_transform)) {
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

fn did_hit(projectile: (&Projectile, &Transform), enemy: (&Enemy, &Transform)) -> bool {
  let projectile_transform = projectile.1;
  let projectile = projectile.0;

  let enemy_transform = enemy.1;
  let enemy = enemy.0;

  let l1 = (
    projectile_transform.translation().x - projectile.width,
    projectile_transform.translation().y - projectile.height,
  );

  let r1 = (
    projectile_transform.translation().x + projectile.width,
    projectile_transform.translation().y + projectile.height,
  );

  let l2 = (
    enemy_transform.translation().x - enemy.width,
    enemy_transform.translation().y - enemy.height,
  );

  let r2 = (
    enemy_transform.translation().x + enemy.width,
    enemy_transform.translation().y + enemy.height,
  );

  if l1.0 > r2.0 || l2.0 > r1.0 {
    return false;
  }

  if l1.1 > r2.1 || l2.1 > r1.1 {
    return false;
  }

  true
}
