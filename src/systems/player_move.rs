use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::incantation_catastrophe::{Player};

pub struct PlayerMoveSystem;

impl<'s> System<'s> for PlayerMoveSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Player>,
  );

  fn run(&mut self, (transforms, players): Self::SystemData) {
    println!("Running");
  }
}