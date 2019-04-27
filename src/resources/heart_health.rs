use amethyst::ecs::Entity;

#[derive(Default)]
pub struct Hearts {
    pub hearts: Vec<Entity>,
}

// impl Default for Hearts {
//     fn default() -> Self {
//     Hearts { hearts: None }
// }
// }