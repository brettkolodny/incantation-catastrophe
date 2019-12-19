use amethyst::core::{
    math::{Unit, Vector3},
    Time, Transform,
};
use amethyst::ecs::{Entities, Join, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;

use crate::components::{
    Bishop, CurrentDirection, CurrentFrame, GameplayItem, Projectile, Size, Speed,
};
use crate::resources::{AnimationSpriteSheets, CurrentState, PlayerResource};

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
        Read<'s, PlayerResource>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, CurrentState>,
        Read<'s, AnimationSpriteSheets>,
        WriteStorage<'s, CurrentFrame>,
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
            player,
            time,
            entities,
            state,
            animation_spritesheets,
            mut frames,
        ): Self::SystemData,
    ) {
        if !state.is_gameplay() {
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
                    sprite_sheet: animation_spritesheets.sprite_sheets["bishop_projectile"].clone(),
                    sprite_number: 0,
                }
            };

            for mut transform in bishop_transforms {
                let direction = {
                    let new_direction = player_transform.translation() - transform.translation();
                    Unit::new_normalize(new_direction)
                };

                let player_x = player_transform.translation().x;
                let player_y = player_transform.translation().y;

                let transform_x = transform.translation().x;
                let transform_y = transform.translation().y;

                let angle = (player_y - transform_y).atan2(player_x - transform_x);

                transform.set_rotation_z_axis(angle);
                transform.set_scale(Vector3::new(1.5, 1.5, 1.5));

                entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(Projectile, &mut projectiles)
                    .with(CurrentDirection::custom(direction), &mut directions)
                    .with(Speed::new(500.), &mut speeds)
                    .with(Size::new(40., 40.), &mut sizes)
                    .with(GameplayItem, &mut gameplay_items)
                    .with(sprite_render.clone(), &mut sprite_renders)
                    .with(CurrentFrame::new(time.absolute_time_seconds()), &mut frames)
                    .build();
            }
        }
    }
}
