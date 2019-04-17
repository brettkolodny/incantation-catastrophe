use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Bishop {
    pub time_since_move: f32,
    pub time_since_shot: f32,
    pub shot_cooldown: f32,
}

impl Component for Bishop {
    type Storage = DenseVecStorage<Self>;
}
