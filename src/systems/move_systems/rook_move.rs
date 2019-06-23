use amethyst::core::{nalgebra, timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use std::f32::consts::{FRAC_PI_2, PI};

use crate::components::{CurrentDirection, Rook, Speed};
use crate::resources::{CurrentState, PlayerResource};

pub struct RookMoveSystem;

impl<'s> System<'s> for RookMoveSystem {
    type SystemData = (
        Read<'s, PlayerResource>,
        ReadStorage<'s, Rook>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, CurrentDirection>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, CurrentState>,
    );

    fn run(
        &mut self,
        (player, rooks, speeds, mut directions, mut transforms, time, state): Self::SystemData,
    ) {
        if state.is_paused() {
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
                let new_vector = nalgebra::base::Matrix::normalize(&new_vector);
                let new_vector = nalgebra::Unit::new_unchecked(new_vector);

                if new_vector.x.abs() > new_vector.y.abs() {
                    if new_vector.x < 0. {
                        direction.turn_right();
                        rook_transform.set_rotation_euler(0., 0., PI + FRAC_PI_2);
                    } else {
                        direction.turn_left();
                        rook_transform.set_rotation_euler(0., 0., FRAC_PI_2);
                    }
                } else {
                    if new_vector.y < 0. {
                        direction.turn_up();
                        rook_transform.set_rotation_euler(0., 0., 0.);
                    } else {
                        direction.turn_down();
                        rook_transform.set_rotation_euler(0., 0., PI);
                    }
                }

                rook_transform
                    .move_along_global(new_vector, time.delta_seconds() * rook_speed.speed);
            }
        }
    }
}
