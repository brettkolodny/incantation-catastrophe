use amethyst::core::{math, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use std::f32::consts::PI;

use crate::components::{CurrentDirection, Pawn, Speed};
use crate::resources::{CurrentState, PlayerResource};

pub struct PawnMoveSystem;

impl<'s> System<'s> for PawnMoveSystem {
    type SystemData = (
        WriteStorage<'s, CurrentDirection>,
        Read<'s, PlayerResource>,
        ReadStorage<'s, Pawn>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (mut directions, player, pawns, speeds, mut transforms, time, state): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        if let Some(player) = player.player {
            let player_transform = transforms.get(player).unwrap().clone();
            for (pawn_transform, pawn_speed, direction, _) in
                (&mut transforms, &speeds, &mut directions, &pawns).join()
            {
                let player_vector = player_transform.translation();
                let pawn_vector = pawn_transform.translation();

                let new_vector = player_vector - pawn_vector;
                let new_vector = math::base::Matrix::normalize(&new_vector);
                let new_vector = math::Unit::new_unchecked(new_vector);

                if player_vector.x < pawn_vector.x {
                    direction.turn_right();
                    pawn_transform.set_rotation_y_axis(PI);
                } else {
                    direction.turn_left();
                    pawn_transform.set_rotation_y_axis(0.);
                }

                pawn_transform
                    .prepend_translation_along(new_vector, time.delta_seconds() * pawn_speed.speed);
            }
        }
    }
}
