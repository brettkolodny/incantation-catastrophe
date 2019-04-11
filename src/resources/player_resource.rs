use amethyst::ecs::Entity;

#[derive(Default)]
pub struct PlayerResource {
    pub player: Option<Entity>,
}
