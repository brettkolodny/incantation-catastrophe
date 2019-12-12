use amethyst::ecs::{Component, DenseVecStorage};

pub struct CurrentFrame {
    pub current_frame: usize,
    pub time: f64,
}

impl CurrentFrame {
    pub fn new(time: f64) -> Self {
        CurrentFrame {
            current_frame: 0,
            time,
        }
    }
}

impl Default for CurrentFrame {
    fn default() -> Self {
        CurrentFrame {
            current_frame: 0,
            time: 0.,
        }
    }
}

impl Component for CurrentFrame {
    type Storage = DenseVecStorage<Self>;
}
