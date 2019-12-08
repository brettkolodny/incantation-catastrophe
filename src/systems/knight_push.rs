use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{CurrentDirection, Direction, Knight, Size, Speed};
use crate::resources::PlayerResource;
use crate::utility::did_hit;

pub struct KnightPushSystem;

impl<'s> System<'s> for KnightPushSystem {
    type SystemData = (
        ReadStorage<'s, Knight>,
        ReadStorage<'s, CurrentDirection>,
        ReadStorage<'s, Speed>,
        ReadStorage<'s, Size>,
        WriteStorage<'s, Transform>,
        Read<'s, PlayerResource>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (knights, directions, speeds, sizes, mut transforms, player_resource, time): Self::SystemData,
    ) {
        if let Some(player) = player_resource.player {
            let mut player_transform_new: Option<Transform> = None;
            let player_size = sizes.get(player).unwrap();
            for (_, direction, speed, size, transform) in
                (&knights, &directions, &speeds, &sizes, &transforms).join()
            {
                let player_transform = transforms.get(player).unwrap();
                if did_hit((player_size, &player_transform), (size, transform)) {
                    let mut player_transform = transforms.get(player).unwrap().clone();
                    if let Direction::Custom(vector) = direction.current_direction {
                        player_transform
                            .prepend_translation_along(vector, time.delta_seconds() * speed.speed);
                        player_transform_new = Some(player_transform);
                    }
                }
            }

            if let Some(new_transform) = player_transform_new {
                let player_transform = transforms.get_mut(player).unwrap();
                let (x, y, z) = {
                    let translation = new_transform.translation();
                    let x = translation.x;
                    let y = translation.y;
                    let z = translation.z;

                    (x, y, z)
                };
                player_transform.set_translation_xyz(x, y, z);
            }
        }
    }
}
