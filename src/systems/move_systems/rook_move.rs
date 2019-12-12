use amethyst::core::{math, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use std::f32::consts::PI;

use crate::components::{CurrentDirection, Rook, Speed};
use crate::resources::{CurrentState, PlayerResource};

pub struct RookMoveSystem;

impl<'s> System<'s> for RookMoveSystem {
    type SystemData = (
        WriteStorage<'s, CurrentDirection>,
        Read<'s, PlayerResource>,
        ReadStorage<'s, Rook>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (mut directions, player, rooks, speeds, mut transforms, time, state): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            return;
        }

        if let Some(player) = player.player {
            let player_transform = transforms.get(player).unwrap().clone();
            for (rook_transform, rook_speed, direction, _) in
                (&mut transforms, &speeds, &mut directions, &rooks).join()
            {
                let player_vector = player_transform.translation();
                let rook_vector = rook_transform.translation();

                let new_vector = player_vector - rook_vector;
                let new_vector = math::base::Matrix::normalize(&new_vector);
                let new_vector = math::Unit::new_unchecked(new_vector);

                if player_vector.x < rook_vector.x {
                    direction.turn_right();
                    rook_transform.set_rotation_y_axis(PI);
                } else {
                    direction.turn_left();
                    rook_transform.set_rotation_y_axis(0.);
                }

                rook_transform
                    .prepend_translation_along(new_vector, time.delta_seconds() * rook_speed.speed);
            }
        }
    }
}
