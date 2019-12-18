use crate::components::{CurrentFrame, Projectile};
use crate::resources::CurrentState;
use amethyst::core::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::renderer::SpriteRender;

pub struct BishopProjectileAnimationSystem;

impl<'s> System<'s> for BishopProjectileAnimationSystem {
    type SystemData = (
        ReadStorage<'s, Projectile>,
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, CurrentState>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (projectiles, mut frames, mut sprite_renders, state, time): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            return;
        }

        for (_, frame, sprite_render) in (&projectiles, &mut frames, &mut sprite_renders).join() {
            if time.absolute_time_seconds() - frame.time > 0.02 {
                if frame.current_frame == 44 {
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
