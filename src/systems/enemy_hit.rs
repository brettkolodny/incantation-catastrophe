use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::components::{Bishop, Enemy, Health, Pawn, PlayerProjectile, Projectile, Rook, Size};
use crate::resources::ScoreResource;
use crate::utility::did_hit;

pub struct EnemyHitSystem;

impl<'s> System<'s> for EnemyHitSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, PlayerProjectile>,
        ReadStorage<'s, Size>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Bishop>,
        ReadStorage<'s, Pawn>,
        ReadStorage<'s, Rook>,
        Write<'s, ScoreResource>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            transforms,
            projectiles,
            player_projectiles,
            sizes,
            mut healths,
            enemies,
            bishops,
            pawns,
            rooks,
            mut score,
            entities,
        ): Self::SystemData,
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
            for (enemy_transform, _, mut enemy_health, enemy_size, enemy_entity) in
                (&transforms, &enemies, &mut healths, &sizes, &entities).join()
            {
                if did_hit(
                    (projectile_size, projectile_transform),
                    (enemy_size, enemy_transform),
                ) {
                    enemy_health.health -= 1;

                    if enemy_health.health == 0 {
                        if let Some(_) = pawns.get(enemy_entity) {
                            score.score += 1;
                        } else if let Some(_) = bishops.get(enemy_entity) {
                            score.score += 5;
                        } else if let Some(_) = rooks.get(enemy_entity) {
                            score.score += 10;
                        }

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
