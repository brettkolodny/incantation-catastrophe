use amethyst::ecs::{Component, DenseVecStorage};

pub struct Health {
    pub health: u32,
}

impl Health {
    pub fn _new(health: u32) -> Self {
        Health { health }
    }

    pub fn pawn() -> Self {
        Health { health: 1 }
    }

    pub fn bishop() -> Self {
        Health { health: 3 }
    }
}

impl Default for Health {
    fn default() -> Self {
        Health { health: 10 }
    }
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
