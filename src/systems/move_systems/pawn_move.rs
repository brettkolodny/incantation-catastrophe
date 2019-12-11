use amethyst::core::{math, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{Pawn, Speed};
use crate::resources::{CurrentState, PlayerResource};

pub struct PawnMoveSystem;

impl<'s> System<'s> for PawnMoveSystem {
    type SystemData = (
        Read<'s, PlayerResource>,
        ReadStorage<'s, Pawn>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(&mut self, (player, pawns, speeds, mut transforms, time, state): Self::SystemData) {
        if state.is_paused() {
            return;
        }

        if let Some(player) = player.player {
            let player_transform = transforms.get(player).unwrap().clone();
            for (pawn_transform, pawn_speed, _) in (&mut transforms, &speeds, &pawns).join() {
                let player_vector = player_transform.translation();
                let pawn_vector = pawn_transform.translation();

                let new_vector = player_vector - pawn_vector;
                let new_vector = math::base::Matrix::normalize(&new_vector);
                let new_vector = math::Unit::new_unchecked(new_vector);

                pawn_transform
                    .prepend_translation_along(new_vector, time.delta_seconds() * pawn_speed.speed);
            }
        }
    }
}
