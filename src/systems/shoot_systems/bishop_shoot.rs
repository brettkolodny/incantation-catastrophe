use amethyst::core::{nalgebra::Unit, Time, Transform};
use amethyst::ecs::{Entities, Join, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;

use crate::components::{Bishop, CurrentDirection, GameplayItem, Projectile, Size, Speed};
use crate::resources::{PlayerResource, SpriteSheet, CurrentState};

pub struct BishopShootSystem;

impl<'s> System<'s> for BishopShootSystem {
    type SystemData = (
        WriteStorage<'s, Bishop>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, GameplayItem>,
        Read<'s, SpriteSheet>,
        Read<'s, PlayerResource>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (
            mut bishops,
            mut projectiles,
            mut transforms,
            mut speeds,
            mut sizes,
            mut directions,
            mut sprite_renders,
            mut gameplay_items,
            spritesheet,
            player,
            time,
            entities,
            state,
        ): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        if let Some(player) = player.player {
            let player_transform = transforms.get(player).unwrap().clone();
            let mut bishop_transforms: Vec<Transform> = Vec::default();
            for (bishop, transform) in (&mut bishops, &transforms).join() {
                if bishop.time_since_shot >= bishop.shot_cooldown {
                    bishop_transforms.push(transform.clone());
                    bishop.time_since_shot = 0.;
                } else {
                    bishop.time_since_shot += time.delta_seconds();
                }
            }

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
                    sprite_number: 3,
                }
            };

            for mut transform in bishop_transforms {
                let direction = {
                    let new_direction = player_transform.translation() - transform.translation();
                    Unit::new_normalize(new_direction)
                };

                transform.set_scale(2., 2., 1.);
                entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(Projectile, &mut projectiles)
                    .with(CurrentDirection::custom(direction), &mut directions)
                    .with(Speed::new(500.), &mut speeds)
                    .with(Size::new(52., 52.), &mut sizes)
                    .with(sprite_render.clone(), &mut sprite_renders)
                    .with(GameplayItem, &mut gameplay_items)
                    .build();
            }
        }
    }
}
