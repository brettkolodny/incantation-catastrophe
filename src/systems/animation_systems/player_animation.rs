use crate::components::{CurrentFrame, Player};
use crate::resources::CurrentState;
use amethyst::core::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

pub struct PlayerAnimationSystem;

impl<'s> System<'s> for PlayerAnimationSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, CurrentState>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (input, players, mut frames, mut sprite_renders, state, time): Self::SystemData,
    ) {
        if state.is_paused() {
            return;
        }

        let horizontal_movement = input.axis_value("horizontal");
        let vertical_movement = input.axis_value("vertical");

        for (_, frame, sprite_render) in (&players, &mut frames, &mut sprite_renders).join() {
            if horizontal_movement != Some(0.) || vertical_movement != Some(0.) {
                if time.absolute_time_seconds() - frame.time > 0.15 {
                    if frame.current_frame < 4 {
                        sprite_render.sprite_number = 4;
                        frame.current_frame = 4;
                    } else if frame.current_frame == 7 {
                        sprite_render.sprite_number = 4;
                        frame.current_frame = 4;
                    } else {
                        sprite_render.sprite_number = frame.current_frame + 1;
                        frame.current_frame += 1;
                    }

                    frame.time = time.absolute_time_seconds();
                }
            } else {
                if time.absolute_time_seconds() - frame.time > 0.15 {
                    if frame.current_frame > 3 {
                        sprite_render.sprite_number = 0;
                        frame.current_frame = 0;
                    } else if frame.current_frame == 7 {
                        sprite_render.sprite_number = 4;
                        frame.current_frame = 4;
                    } else {
                        sprite_render.sprite_number = frame.current_frame + 1;
                        frame.current_frame += 1;
                    }

                    frame.time = time.absolute_time_seconds();
                }
            }
        }
    }
}
