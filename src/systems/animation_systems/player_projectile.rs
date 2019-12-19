use crate::components::{CurrentFrame, PlayerProjectile};
use crate::resources::CurrentState;
use amethyst::core::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::renderer::SpriteRender;

pub struct PlayerProjectileAnimationSystem;

impl<'s> System<'s> for PlayerProjectileAnimationSystem {
    type SystemData = (
        ReadStorage<'s, PlayerProjectile>,
        WriteStorage<'s, CurrentFrame>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, CurrentState>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (player_projectiles, mut frames, mut sprite_renders, state, time): Self::SystemData,
    ) {
        if !state.is_gameplay() {
            return;
        }

        for (_, frame, sprite_render) in
            (&player_projectiles, &mut frames, &mut sprite_renders).join()
        {
            if time.absolute_time_seconds() - frame.time > 0.1 {
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
