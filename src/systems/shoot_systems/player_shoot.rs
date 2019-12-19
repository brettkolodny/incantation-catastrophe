use amethyst::core::{math::Vector3, timing::Time, Transform};
use amethyst::ecs::{Entities, Join, System};
use amethyst::ecs::{Read, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

use std::f32::consts::{FRAC_PI_2, PI};

use crate::components::{
    CurrentDirection, CurrentFrame, Direction, GameplayItem, Player, PlayerProjectile, Projectile,
    Size, Speed,
};
use crate::resources::{AnimationSpriteSheets, CurrentState};

pub struct PlayerShootSystem {
    pub is_shooting: bool,
}

impl<'s> System<'s> for PlayerShootSystem {
    type SystemData = (
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, GameplayItem>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, PlayerProjectile>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, AnimationSpriteSheets>,
        Entities<'s>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (
            mut frames,
            mut gameplay_items,
            mut projectiles,
            mut player_projectiles,
            mut players,
            mut transforms,
            mut speeds,
            mut sizes,
            mut directions,
            mut sprite_renders,
            animation_spritesheets,
            entities,
            input,
            time,
            state,
        ): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            return;
        }

        if let Some(true) = input.action_is_down("shoot") {
            let mut player_transforms_directions: Vec<(Transform, CurrentDirection)> = Vec::new();

            for (mut player, transform, direction) in
                (&mut players, &transforms, &directions).join()
            {
                if !self.is_shooting {
                    player_transforms_directions.push((transform.clone(), direction.clone()));
                    player.time_since_shot = 0.;
                    self.is_shooting = true;
                } else {
                    player.time_since_shot += time.delta_seconds();
                }
            }

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: animation_spritesheets.sprite_sheets["player_projectile"].clone(),
                    sprite_number: 0,
                }
            };

            for (mut transform, direction) in player_transforms_directions {
                transform.set_scale(Vector3::new(1.2, 1.2, 1.));

                match direction.current_direction {
                    Direction::Right => transform.set_rotation_euler(0., 0., 0.),
                    Direction::Left => transform.set_rotation_euler(0., 0., PI),
                    Direction::Up => transform.set_rotation_euler(0., 0., FRAC_PI_2),
                    Direction::Down => transform.set_rotation_euler(0., 0., PI + FRAC_PI_2),
                    _ => &transform,
                };

                entities
                    .build_entity()
                    .with(GameplayItem::default(), &mut gameplay_items)
                    .with(PlayerProjectile::default(), &mut player_projectiles)
                    .with(Projectile, &mut projectiles)
                    .with(Size::new(16., 16.), &mut sizes)
                    .with(transform, &mut transforms)
                    .with(direction, &mut directions)
                    .with(sprite_render.clone(), &mut sprite_renders)
                    .with(Speed::new(750.), &mut speeds)
                    .with(CurrentFrame::new(time.absolute_time_seconds()), &mut frames)
                    .build();
            }
        } else {
            self.is_shooting = false;
        }
    }
}
