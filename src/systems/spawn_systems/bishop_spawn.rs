use amethyst::core::{math::Vector3, Time, Transform};
use amethyst::ecs::{Entities, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;
use rand::Rng;

use std::f32::consts::PI;

use crate::components::{Bishop, CurrentFrame, Enemy, GameplayItem, Health, Size};
use crate::resources::{AnimationSpriteSheets, CurrentState};
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH, RADIUS};

pub struct BishopSpawnSystem {
    pub spawn_timer: f32,
    pub time_since_spawn: f32,
}

impl<'s> System<'s> for BishopSpawnSystem {
    type SystemData = (
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, GameplayItem>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Bishop>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Health>,
        Read<'s, AnimationSpriteSheets>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (
            mut frames,
            mut gameplay_items,
            mut enemies,
            mut bishops,
            mut sizes,
            mut sprite_renders,
            mut transforms,
            mut healths,
            animation_spritesheets,
            time,
            entities,
            state,
        ): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            if self.time_since_spawn != 0. && state.is_gameover() {
                self.time_since_spawn = 0.;
            }

            return;
        }

        if self.time_since_spawn >= self.spawn_timer {
            let angle = (rand::thread_rng().gen_range(0, 99) as f32 / 100.) * 2. * PI;
            let r = RADIUS * (rand::thread_rng().gen_range(0, 99) as f32 / 100.).sqrt();

            let x = r * angle.cos() + GAMEPLAY_AREA_WIDTH / 2.;
            let y = r * angle.sin() - GAMEPLAY_AREA_HEIGHT / 2.;

            let mut bishop_transform = Transform::default();
            bishop_transform.set_scale(Vector3::new(2.5, 2.5, 1.));
            bishop_transform.set_translation_x(x);
            bishop_transform.set_translation_y(y);
            bishop_transform.set_translation_z(-1.);

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: animation_spritesheets.sprite_sheets["bishop"].clone(),
                    sprite_number: 0,
                }
            };

            entities
                .build_entity()
                .with(
                    Bishop {
                        time_since_move: 0.,
                        time_since_shot: 2.,
                        shot_cooldown: 4.,
                    },
                    &mut bishops,
                )
                .with(Enemy, &mut enemies)
                .with(Size::new(32., 36.), &mut sizes)
                .with(bishop_transform, &mut transforms)
                .with(sprite_render, &mut sprite_renders)
                .with(Health::bishop(), &mut healths)
                .with(GameplayItem, &mut gameplay_items)
                .with(CurrentFrame::new(time.absolute_time_seconds()), &mut frames)
                .build();

            self.time_since_spawn = 0.;
        } else {
            self.time_since_spawn += time.delta_seconds();
        }
    }
}
