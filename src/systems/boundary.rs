use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System, Write};

use crate::components::{Background, GameplayItem, Player};
use crate::resources::PlayerResource;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct BoundarySystem;

impl<'s> System<'s> for BoundarySystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        Entities<'s>,
        ReadStorage<'s, GameplayItem>,
        ReadStorage<'s, Background>,
        ReadStorage<'s, Player>,
        Write<'s, PlayerResource>,
    );

    fn run(
        &mut self,
        (transforms, entities, gameplay_items, backgrounds, players, mut player_resource): Self::SystemData,
    ) {
        let radius = GAMEPLAY_AREA_HEIGHT / 2.;

        for (transform, entity, _, _, _) in (
            &transforms,
            &entities,
            &gameplay_items,
            !&backgrounds,
            !&players,
        )
            .join()
        {
            let entity_x = transform.translation().x;
            let entity_y = transform.translation().y;

            if (entity_x - (GAMEPLAY_AREA_WIDTH / 2.)).powi(2)
                + (entity_y - (GAMEPLAY_AREA_HEIGHT / 2.)).powi(2)
                > radius.powi(2)
            {
                if let Err(e) = entities.delete(entity) {
                    dbg!(e);
                }
            }
        }

        if let Some(player) = player_resource.player {
            let (player_x, player_y) = {
                let player_transform = transforms.get(player).unwrap();
                (
                    player_transform.translation().x,
                    player_transform.translation().y,
                )
            };

            if (player_x - (GAMEPLAY_AREA_WIDTH / 2.)).powi(2)
                + (player_y - (GAMEPLAY_AREA_HEIGHT / 2.)).powi(2)
                > radius.powi(2)
            {
                player_resource.player = None;
                if let Err(e) = entities.delete(player) {
                    dbg!(e);
                }
            }
        };
    }
}
