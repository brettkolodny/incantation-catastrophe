use crate::components::{Background, GameplayItem, Health, Potion, Size};
use crate::resources::{CurrentState, Hearts, PlayerResource, SpriteSheet};
use crate::utility::did_hit;
use amethyst::core::{math::Vector3, Transform};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::renderer::SpriteRender;

pub struct PotionPickupSystem;

impl<'s> System<'s> for PotionPickupSystem {
    type SystemData = (
        Write<'s, PlayerResource>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Size>,
        WriteStorage<'s, Health>,
        Write<'s, Hearts>,
        Entities<'s>,
        Read<'s, CurrentState>,
        ReadStorage<'s, Potion>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, SpriteSheet>,
        WriteStorage<'s, Background>,
        WriteStorage<'s, GameplayItem>,
    );

    fn run(
        &mut self,
        (
            player,
            mut transforms,
            sizes,
            mut healths,
            mut hearts,
            entities,
            state,
            potions,
            mut sprite_renders,
            sprite_sheet,
            mut backgrounds,
            mut gameplay_items,
        ): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            return;
        }

        if let Some(player) = player.player {
            let player_info = { (sizes.get(player).unwrap(), transforms.get(player).unwrap()) };
            let mut player_health = healths.get_mut(player).unwrap();

            let mut potion_picked_up = false;

            for (size, transform, _, potion_entity) in
                (&sizes, &transforms, &potions, &entities).join()
            {
                if player_health.health != 10 && did_hit(player_info, (&size, &transform)) {
                    player_health.health += 1;

                    if let Err(e) = entities.delete(potion_entity) {
                        dbg!(e);
                    }

                    potion_picked_up = true;
                }
            }

            if potion_picked_up {
                let current_hearts = &mut hearts.hearts;
                let last_heart_x = transforms
                    .get(*current_hearts.last().unwrap())
                    .unwrap()
                    .translation()
                    .x;
                let mut new_heart_transform = Transform::default();
                new_heart_transform.set_translation_xyz(last_heart_x + 50., -50., 0.);
                new_heart_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));

                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet.sprite_sheet.clone().unwrap(),
                    sprite_number: 2,
                };

                current_hearts.push(
                    entities
                        .build_entity()
                        .with(new_heart_transform, &mut transforms)
                        .with(sprite_render, &mut sprite_renders)
                        .with(Background, &mut backgrounds)
                        .with(GameplayItem, &mut gameplay_items)
                        .build(),
                );
            }
        }
    }
}
