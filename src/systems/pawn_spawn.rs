use amethyst::core::{Time, Transform};
use amethyst::ecs::{Entities, Read, System, WriteStorage};
use amethyst::renderer::SpriteRender;
use rand::Rng;

use crate::components::{Enemy, Speed};
use crate::resources::SpriteSheet;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct PawnSpawnSystem {
  pub spawn_timer: f32,
  pub time_since_spawn: f32,
}

impl<'s> System<'s> for PawnSpawnSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Speed>,
    WriteStorage<'s, Enemy>,
    WriteStorage<'s, SpriteRender>,
    Read<'s, SpriteSheet>,
    Read<'s, Time>,
    Entities<'s>,
  );

  fn run(
    &mut self,
    (mut transforms, mut speeds, mut enemies, mut sprite_renders, spritesheet, time, entities): Self::SystemData,
  ) {
    if self.time_since_spawn >= self.spawn_timer {
      let radius = (GAMEPLAY_AREA_HEIGHT) / 2.;
      let angle = rand::thread_rng().gen_range(0, 360) as f32;

      let (x, y) = (
        (radius * angle.sin() + GAMEPLAY_AREA_WIDTH / 2.),
        (radius * angle.cos() + GAMEPLAY_AREA_HEIGHT / 2.),
      );
      let mut local_transform = Transform::default();
      local_transform.set_xyz(x, y, 0.);

      let sprite_render = {
        SpriteRender {
          sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
          sprite_number: 1,
        }
      };

      dbg!("Building entity");
      entities
        .build_entity()
        .with(local_transform, &mut transforms)
        .with(sprite_render, &mut sprite_renders)
        .with(Speed::default(), &mut speeds)
        .with(Enemy::pawn(), &mut enemies)
        .build();

      self.time_since_spawn = 0.;
    } else {
      self.time_since_spawn += time.delta_seconds();
    }
  }
}
