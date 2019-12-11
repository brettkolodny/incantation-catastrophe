use amethyst::core::{math, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{Rook, Speed};
use crate::resources::{CurrentState, PlayerResource};

pub struct RookMoveSystem;

impl<'s> System<'s> for RookMoveSystem {
    type SystemData = (
        Read<'s, PlayerResource>,
        ReadStorage<'s, Rook>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (player, rooks, speeds, mut transforms, time, state): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        if let Some(player) = player.player {
            let player_transform = transforms.get(player).unwrap().clone();
            for (rook_transform, rook_speed, _) in
                (&mut transforms, &speeds, &rooks).join()
            {
                let player_vector = player_transform.translation();
                let rook_vector = rook_transform.translation();

                let new_vector = player_vector - rook_vector;
                let new_vector = math::base::Matrix::normalize(&new_vector);
                let new_vector = math::Unit::new_unchecked(new_vector);

                rook_transform
                    .prepend_translation_along(new_vector, time.delta_seconds() * rook_speed.speed);
            }
        }
    }
}
