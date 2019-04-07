use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Entities, Join, System};
use amethyst::ecs::{Read, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use crate::components::{
  CurrentDirection, GameplayItem, Player, PlayerProjectile, Projectile, Speed, Size,
};
use crate::resources::SpriteSheet;

pub struct PlayerShootSystem {
  pub is_shooting: bool,
}

impl<'s> System<'s> for PlayerShootSystem {
  type SystemData = (
    WriteStorage<'s, GameplayItem>,
    WriteStorage<'s, Projectile>,
    WriteStorage<'s, PlayerProjectile>,
    WriteStorage<'s, Player>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Speed>,
    WriteStorage<'s, Size>,
    WriteStorage<'s, CurrentDirection>,
    WriteStorage<'s, SpriteRender>,
    Read<'s, SpriteSheet>,
    Entities<'s>,
    Read<'s, InputHandler<String, String>>,
    Read<'s, Time>,
  );

  fn run(
    &mut self,
    (
      mut gameplay_items,
      mut projectiles,
      mut player_projectiles,
      mut players,
      mut transforms,
      mut speeds,
      mut sizes,
      mut directions,
      mut sprite_renders,
      spritesheet,
      entities,
      input,
      time,
    ): Self::SystemData,
  ) {
    if let Some(true) = input.action_is_down("shoot") {
      let mut player_transforms_directions = Vec::<(Transform, CurrentDirection)>::new();

      for (mut player, transform, direction) in (&mut players, &transforms, &directions).join() {
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
          sprite_sheet: spritesheet.sprite_sheet.clone().unwrap(),
          sprite_number: 3,
        }
      };

      for (mut transform, direction) in player_transforms_directions {
        transform.set_scale(0.75, 0.75, 1.);
        entities
          .build_entity()
          .with(GameplayItem::default(), &mut gameplay_items)
          .with(PlayerProjectile::default(), &mut player_projectiles)
          .with(Projectile, &mut projectiles)
          .with(Size::new(16., 16.), &mut sizes)
          .with(transform, &mut transforms)
          .with(direction, &mut directions)
          .with(sprite_render.clone(), &mut sprite_renders)
          .with(Speed::default(), &mut speeds)
          .build();
      }
    } else {
      self.is_shooting = false;
    }
  }
}
