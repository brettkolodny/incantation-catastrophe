use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{Background, Health, Player, PlayerProjectile, Size};
use crate::resources::PlayerResource;
use crate::utility::did_hit;

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
        Read<'s, Time>,
        Read<'s, PlayerResource>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Size>,
        ReadStorage<'s, PlayerProjectile>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Background>,
        WriteStorage<'s, Health>,
    );

    fn run(
        &mut self,
        (time, player, transforms, sizes, player_projectiles, players, backgrounds, mut healths): Self::SystemData,
    ) {
        if let Some(player) = player.player {
            let player_info = { (sizes.get(player).unwrap(), transforms.get(player).unwrap()) };
            let mut player_health = healths.get_mut(player).unwrap();
            dbg!(player_health.health);

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
                    dbg!("Player was hit!");
                    player_health.health -= 1;
                    self.player_immune = true;
                }
            }

            if self.player_immune {
                dbg!("Player is Immune");
                self.time_since_hit += time.delta_seconds();
                if self.time_since_hit >= self.player_immune_time {
                    self.player_immune = false;
                    self.time_since_hit = 0.;
                }
            }
        }
    }
}
