use amethyst::core::{Time, Transform};
use amethyst::ecs::{Entities, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;
use rand::Rng;

use crate::components::{CurrentDirection, Enemy, Health, Rook, Size, Speed};
use crate::resources::{CurrentState, SpriteSheet};
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct RookSpawnSystem {
    pub spawn_timer: f32,
    pub time_since_spawn: f32,
}

impl<'s> System<'s> for RookSpawnSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, Size>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, Rook>,
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
            mut transforms,
            mut speeds,
            mut sizes,
            mut directions,
            mut rooks,
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
            let radius = (GAMEPLAY_AREA_HEIGHT) / 2.;
            let angle = rand::thread_rng().gen_range(0, 360) as f32;

            let (x, y) = (
                (radius * angle.sin() + GAMEPLAY_AREA_WIDTH / 2.),
                (radius * angle.cos() + GAMEPLAY_AREA_HEIGHT / 2.),
            );

            let mut local_transform = Transform::default();
            local_transform.set_xyz(x, y, 0.);
            local_transform.set_scale(3., 3., 1.);

            let sprite_render = {
                SpriteRender {
                    sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
                    sprite_number: 6,
                }
            };

            entities
                .build_entity()
                .with(local_transform, &mut transforms)
                .with(sprite_render, &mut sprite_renders)
                .with(CurrentDirection::default(), &mut directions)
                .with(Size::new(64., 64.), &mut sizes)
                .with(Speed::new(50.), &mut speeds)
                .with(Rook, &mut rooks)
                .with(Enemy, &mut enemies)
                .with(Health::rook(), &mut healths)
                .build();

            self.time_since_spawn = 0.;
        } else {
            self.time_since_spawn += time.delta_seconds();
        }
    }
}
