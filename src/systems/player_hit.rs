use crate::components::{Background, Health, Player, PlayerProjectile, Size};
use crate::resources::{CurrentState, Hearts, PlayerResource};
use crate::utility::did_hit;
use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::renderer::{palette::Srgba, resources::Tint};

pub struct PlayerHitSystem {
    player_immune: bool,
    player_immune_time: f32,
    time_since_hit: f32,
}

impl Default for PlayerHitSystem {
    fn default() -> Self {
        PlayerHitSystem {
            player_immune: false,
            player_immune_time: 3.,
            time_since_hit: 0.,
        }
    }
}

impl<'s> System<'s> for PlayerHitSystem {
    type SystemData = (
        WriteStorage<'s, Tint>,
        Read<'s, Time>,
        Write<'s, PlayerResource>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Size>,
        ReadStorage<'s, PlayerProjectile>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Background>,
        WriteStorage<'s, Health>,
        Write<'s, Hearts>,
        Entities<'s>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (
            mut tints,
            time,
            player,
            transforms,
            sizes,
            player_projectiles,
            players,
            backgrounds,
            mut healths,
            mut hearts,
            entities,
            state,
        ): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            if state.is_gameover() {
                self.player_immune = false;
                self.time_since_hit = 0.;
            }

            return;
        }

        if let Some(player) = player.player {
            let player_info = { (sizes.get(player).unwrap(), transforms.get(player).unwrap()) };
            let mut player_health = healths.get_mut(player).unwrap();

            for (size, transform, _, _, _) in (
                &sizes,
                &transforms,
                !&player_projectiles,
                !&players,
                !&backgrounds,
            )
                .join()
            {
                if !self.player_immune && did_hit(player_info, (&size, &transform)) {
                    let current_hearts = &mut hearts.hearts;
                    player_health.health -= 1;
                    self.player_immune = true;

                    if let Err(e) = entities.delete(current_hearts.pop().unwrap()) {
                        dbg!(e);
                    }
                }
            }

            if self.player_immune {
                self.time_since_hit += time.delta_seconds();
                let tint = tints.get_mut(player).unwrap();

                if self.time_since_hit >= self.player_immune_time {
                    self.player_immune = false;
                    self.time_since_hit = 0.;
                    tint.0 = Srgba::new(1., 1., 1., 1.);
                } else {
                    if self.time_since_hit % 0.5 < 0.25 {
                        tint.0 = Srgba::new(0.5, 0.5, 0.5, 1.);
                    } else {
                        tint.0 = Srgba::new(1., 1., 1., 1.);
                    }
                }
            }
        }
    }
}
