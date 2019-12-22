use amethyst::core::{math::Vector3, Transform};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::renderer::SpriteRender;

use rand::prelude::*;

use crate::components::{
    Bishop, Enemy, GameplayItem, Health, Pawn, PlayerProjectile, Potion, Projectile, Rook, Size,
};
use crate::resources::{ScoreResource, SpriteSheet};
use crate::utility::{did_hit, FLASK_SPRITE_NUMBER};

pub struct EnemyHitSystem;

impl<'s> System<'s> for EnemyHitSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, PlayerProjectile>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Bishop>,
        ReadStorage<'s, Pawn>,
        ReadStorage<'s, Rook>,
        Write<'s, ScoreResource>,
        WriteStorage<'s, GameplayItem>,
        Read<'s, SpriteSheet>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Potion>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            projectiles,
            player_projectiles,
            mut sizes,
            mut healths,
            enemies,
            bishops,
            pawns,
            rooks,
            mut score,
            mut gameplay_items,
            spritesheet,
            mut sprite_renders,
            mut potions,
            entities,
        ): Self::SystemData,
    ) {
        let mut potion_transform = None;

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
                        let mut rng = rand::thread_rng();
                        let drop_roll: f32 = rng.gen();

                        let mut drop_potion = false;

                        if pawns.get(enemy_entity).is_some() {
                            score.score += 1;
                            if drop_roll > 0.95 {
                                drop_potion = true;
                            }
                        } else if bishops.get(enemy_entity).is_some() {
                            score.score += 5;
                            if drop_roll > 0.5 {
                                drop_potion = true;
                            }
                        } else if rooks.get(enemy_entity).is_some() {
                            score.score += 10;
                            if drop_roll > 0.75 {
                                drop_potion = true;
                            }
                        }

                        if drop_potion {
                            potion_transform = Some(enemy_transform.clone());
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
        if let Some(mut transform) = potion_transform {
            transform.set_scale(Vector3::new(2.5, 2.5, 1.));
            transform.set_translation_z(-3.);

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
                    sprite_number: FLASK_SPRITE_NUMBER,
                }
            };

            entities
                .build_entity()
                .with(transform, &mut transforms)
                .with(sprite_render, &mut sprite_renders)
                .with(Size::new(16., 16.), &mut sizes)
                .with(GameplayItem, &mut gameplay_items)
                .with(Potion, &mut potions)
                .build();
        }
    }
}
