use amethyst::core::{
    math::{Unit, Vector3},
    Time, Transform,
};
use amethyst::ecs::{Entities, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;
use rand::Rng;

use crate::components::{CurrentDirection, CurrentFrame, Enemy, GameplayItem, Knight, Size, Speed};
use crate::resources::{AnimationSpriteSheets, CurrentState, PlayerResource};
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH, RADIUS};

pub struct KnightSpawnSystem {
    pub spawn_timer: f32,
    pub time_since_spawn: f32,
}

impl<'s> System<'s> for KnightSpawnSystem {
    type SystemData = (
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, GameplayItem>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, Knight>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, AnimationSpriteSheets>,
        Read<'s, PlayerResource>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (
            mut frames,
            mut gameplay_items,
            mut transforms,
            mut speeds,
            mut directions,
            mut sizes,
            mut knights,
            mut enemies,
            mut sprite_renders,
            animation_spritesheets,
            player,
            time,
            entities,
            state,
        ): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        if self.time_since_spawn >= self.spawn_timer {
            if let Some(player) = player.player {
                let angle = rand::thread_rng().gen_range(0, 360) as f32;

                let (x, y) = (
                    (RADIUS * angle.sin() + GAMEPLAY_AREA_WIDTH / 2.),
                    (RADIUS * angle.cos() + GAMEPLAY_AREA_HEIGHT / -2.),
                );

                let mut local_transform = Transform::default();
                local_transform.set_translation_xyz(x, y, 0.);
                local_transform.set_scale(Vector3::new(3., 3., 1.0));

                let sprite_render = {
                    SpriteRender {
                        sprite_sheet: animation_spritesheets.sprite_sheets["knight"].clone(),
                        sprite_number: 0,
                    }
                };

                let direction = {
                    let player_transform = transforms.get(player).unwrap();
                    let new_direction =
                        player_transform.translation() - local_transform.translation();
                    Unit::new_normalize(new_direction)
                };

                entities
                    .build_entity()
                    .with(local_transform, &mut transforms)
                    .with(sprite_render, &mut sprite_renders)
                    .with(Size::new(16., 28.), &mut sizes)
                    .with(Speed::new(600.), &mut speeds)
                    .with(CurrentDirection::custom(direction), &mut directions)
                    .with(Knight, &mut knights)
                    .with(Enemy, &mut enemies)
                    .with(GameplayItem, &mut gameplay_items)
                    .with(CurrentFrame::new(time.absolute_time_seconds()), &mut frames)
                    .build();

                self.time_since_spawn = 0.;
            }
        } else {
            self.time_since_spawn += time.delta_seconds();
        }
    }
}
