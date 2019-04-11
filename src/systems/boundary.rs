use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System};

use crate::components::{Background, GameplayItem};
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct BoundarySystem;

impl<'s> System<'s> for BoundarySystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        Entities<'s>,
        ReadStorage<'s, GameplayItem>,
        ReadStorage<'s, Background>,
    );

    fn run(&mut self, (transforms, entities, gameplay_items, backgrounds): Self::SystemData) {
        for (transform, entity, _, _) in
            (&transforms, &entities, &gameplay_items, !&backgrounds).join()
        {
            let radius = GAMEPLAY_AREA_HEIGHT / 2.;
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
    }
}
