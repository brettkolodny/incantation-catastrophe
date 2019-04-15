use amethyst::ecs::{Entities, ReadStorage, System, Write};

use crate::components::Health;
use crate::resources::PlayerResource;

pub struct PlayerDeathSystem;

impl<'s> System<'s> for PlayerDeathSystem {
    type SystemData = (
        Write<'s, PlayerResource>,
        ReadStorage<'s, Health>,
        Entities<'s>,
    );

    fn run(&mut self, (mut player, health, entities): Self::SystemData) {
        if let Some(player_entity) = player.player {
            let player_health = Health::default();
            let player_health = health.get(player_entity).unwrap_or(&player_health);

            if player_health.health <= 0 {
                player.player = None;
                if let Err(e) = entities.delete(player_entity) {
                    dbg!(e);
                }
            }
        }
    }
}
