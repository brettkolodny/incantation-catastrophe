use amethyst::core::{math::Vector3, Time, Transform};
use amethyst::ecs::{Entities, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;
use rand::Rng;

use crate::components::{CurrentDirection, CurrentFrame, Enemy, GameplayItem, Health, Rook, Size, Speed};
use crate::resources::{CurrentState, AnimationSpriteSheets};
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH, RADIUS};

pub struct RookSpawnSystem {
    pub spawn_timer: f32,
    pub time_since_spawn: f32,
}

impl<'s> System<'s> for RookSpawnSystem {
    type SystemData = (
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, GameplayItem>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, Rook>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, SpriteRender>,
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
            mut transforms,
            mut speeds,
            mut sizes,
            mut directions,
            mut rooks,
            mut enemies,
            mut sprite_renders,
            mut healths,
            animation_spritesheets,
            time,
            entities,
            state,
        ): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        if self.time_since_spawn >= self.spawn_timer {
            let angle = rand::thread_rng().gen_range(0, 360) as f32;

            let (x, y) = (
                (RADIUS * angle.sin() + GAMEPLAY_AREA_WIDTH / 2.),
                (RADIUS * angle.cos() + GAMEPLAY_AREA_HEIGHT / -2.),
            );

            let mut local_transform = Transform::default();
            local_transform.set_translation_xyz(x, y, 0.);
            local_transform.set_scale(Vector3::new(3., 3., 1.));

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: animation_spritesheets.sprite_sheets["rook"].clone(),
                    sprite_number: 0,
                }
            };

            entities
                .build_entity()
                .with(local_transform, &mut transforms)
                .with(sprite_render, &mut sprite_renders)
                .with(CurrentDirection::default(), &mut directions)
                .with(Size::new(32., 34.), &mut sizes)
                .with(Speed::new(50.), &mut speeds)
                .with(Rook, &mut rooks)
                .with(Enemy, &mut enemies)
                .with(Health::rook(), &mut healths)
                .with(GameplayItem, &mut gameplay_items)
                .with(CurrentFrame::new(time.absolute_time_seconds()), &mut frames)
                .build();

            self.time_since_spawn = 0.;
        } else {
            self.time_since_spawn += time.delta_seconds();
        }
    }
}
