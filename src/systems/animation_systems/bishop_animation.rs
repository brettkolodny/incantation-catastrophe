use crate::components::{Bishop, CurrentFrame};
use crate::resources::CurrentState;
use amethyst::core::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::renderer::SpriteRender;

pub struct BishopAnimationSystem;

impl<'s> System<'s> for BishopAnimationSystem {
    type SystemData = (
        ReadStorage<'s, Bishop>,
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, CurrentState>,
        Read<'s, Time>,
    );

    fn run(&mut self, (pawns, mut frames, mut sprite_renders, state, time): Self::SystemData) {
        if !state.is_gameplay() {
            return;
        }

        for (bishop, frame, sprite_render) in (&pawns, &mut frames, &mut sprite_renders).join() {
            if bishop.time_since_shot > 3. {
                sprite_render.sprite_number = 4;
            } else if time.absolute_time_seconds() - frame.time > 0.15 {
                if frame.current_frame == 3 {
                    sprite_render.sprite_number = 0;
                    frame.current_frame = 0;
                } else {
                    sprite_render.sprite_number = frame.current_frame + 1;
                    frame.current_frame += 1;
                }

                frame.time = time.absolute_time_seconds();
            }
        }
    }
}
