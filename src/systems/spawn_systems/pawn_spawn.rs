use amethyst::core::{math::Vector3, Time, Transform};
use amethyst::ecs::{Entities, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;
use rand::Rng;

use crate::components::{GameplayItem, CurrentDirection, Enemy, Health, Pawn, Size, Speed};
use crate::resources::{CurrentState, SpriteSheet};
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH, PAWN_SPRITE_NUMBER, RADIUS};

pub struct PawnSpawnSystem {
    pub spawn_timer: f32,
    pub time_since_spawn: f32,
}

impl<'s> System<'s> for PawnSpawnSystem {
    type SystemData = (
        WriteStorage<'s, GameplayItem>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, Pawn>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Health>,
        Read<'s, SpriteSheet>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (
            mut gameplay_items,
            mut transforms,
            mut speeds,
            mut sizes,
            mut directions,
            mut pawns,
            mut enemies,
            mut sprite_renders,
            mut healths,
            spritesheet,
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
            local_transform.set_scale(Vector3::new(2.75, 2.75, 1.0));

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
                    sprite_number: PAWN_SPRITE_NUMBER,
                }
            };

            entities
                .build_entity()
                .with(local_transform, &mut transforms)
                .with(sprite_render, &mut sprite_renders)
                .with(CurrentDirection::default(), &mut directions)
                .with(Size::new(32., 32.), &mut sizes)
                .with(Speed::new(100.), &mut speeds)
                .with(Pawn, &mut pawns)
                .with(Enemy, &mut enemies)
                .with(Health::pawn(), &mut healths)
                .with(GameplayItem, &mut gameplay_items)
                .build();

            self.time_since_spawn = 0.;
        } else {
            self.time_since_spawn += time.delta_seconds();
        }
    }
}
