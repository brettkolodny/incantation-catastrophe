use amethyst::ecs::{Join, WriteStorage, ReadStorage, System, Read};
use amethyst::core::Time;
use amethyst::renderer::SpriteRender;
use crate::components::{Knight, CurrentFrame};
use crate::resources::CurrentState;

pub struct KnightRunSystem;

impl<'s> System<'s> for KnightRunSystem {
    type SystemData = (
        ReadStorage<'s, Knight>,
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, CurrentState>, 
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (knights, mut frames, mut sprite_renders, state, time): Self::SystemData,
    ) { 
        if state.is_paused() {
            return;
        }

        for (_, frame, sprite_render) in (&knights, &mut frames, &mut sprite_renders).join() {
           if time.absolute_time_seconds() - frame.time > 0.05 {
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
