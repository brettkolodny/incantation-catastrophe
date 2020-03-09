use amethyst::core::{math::Vector3, timing::Time, Transform};
use amethyst::ecs::{Read, WriteStorage, Entities, System};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

use std::f32::consts::{FRAC_PI_2, PI};

use crate::components::{
    CurrentDirection, CurrentFrame, Direction, GameplayItem, Player, PlayerProjectile, Projectile,
    Size, Speed,
};
use crate::resources::{AnimationSpriteSheets, CurrentState, PlayerResource};

const PROJECTILE_SCALE: f32 = 1.2;

pub struct PlayerShootSystem {
    pub is_shooting: bool,
}

impl<'s> System<'s> for PlayerShootSystem {
    type SystemData = (
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, GameplayItem>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, PlayerProjectile>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, AnimationSpriteSheets>,
        WriteStorage<'s, Player>,
        Entities<'s>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
        Read<'s, PlayerResource>,
    );

    fn run(
        &mut self,
        (
            mut frames,
            mut gameplay_items,
            mut projectiles,
            mut player_projectiles,
            mut transforms,
            mut speeds,
            mut sizes,
            mut directions,
            mut sprite_renders,
            animation_spritesheets,
            mut players,
            entities,
            input,
            time,
            state,
            player_resource,
        ): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            return;
        }

        if player_resource.player.is_none() {
            return;
        }

        let player_entity = player_resource.player.unwrap();

        let mut player = players.get_mut(player_entity).unwrap();


        if let Some(true) = input.action_is_down("shoot") {
            if !self.is_shooting {
                player.time_since_shot = 0.;
                self.is_shooting = true;

                let sprite_render = {
                    SpriteRender {
                        sprite_sheet: animation_spritesheets.sprite_sheets["player_projectile"].clone(),
                        sprite_number: 0,
                    }
                };

                let mut projectile_transform = transforms.get(player_entity).unwrap().clone();
                let projectile_direction = directions.get(player_entity).unwrap().clone();

                projectile_transform.set_scale(Vector3::new(PROJECTILE_SCALE, PROJECTILE_SCALE, 0.));

                match projectile_direction.current_direction {
                    Direction::Right => projectile_transform.set_rotation_euler(0., 0., 0.),
                    Direction::Left => projectile_transform.set_rotation_euler(0., 0., PI),
                    Direction::Up => projectile_transform.set_rotation_euler(0., 0., FRAC_PI_2),
                    Direction::Down => projectile_transform.set_rotation_euler(0., 0., PI + FRAC_PI_2),
                    _ => &projectile_transform,
                };

                entities
                    .build_entity()
                    .with(GameplayItem::default(), &mut gameplay_items)
                    .with(PlayerProjectile::default(), &mut player_projectiles)
                    .with(Projectile, &mut projectiles)
                    .with(Size::new(16., 16.), &mut sizes)
                    .with(projectile_transform, &mut transforms)
                    .with(projectile_direction, &mut directions)
                    .with(sprite_render.clone(), &mut sprite_renders)
                    .with(Speed::new(750.), &mut speeds)
                    .with(CurrentFrame::new(time.absolute_time_seconds()), &mut frames)
                    .build();

                } else {
                    player.time_since_shot += time.delta_seconds();
            }

        } else {
            self.is_shooting = false;
        }
    }
}
