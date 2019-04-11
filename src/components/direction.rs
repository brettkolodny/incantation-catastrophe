use amethyst::core::nalgebra::{Unit, Vector3};
use amethyst::ecs::prelude::{Component, VecStorage};

#[derive(Copy, Clone)]
pub struct CurrentDirection {
    pub current_direction: Direction,
}

impl CurrentDirection {
    pub fn turn_up(&mut self) {
        self.current_direction = Direction::Up;
    }

    pub fn turn_down(&mut self) {
        self.current_direction = Direction::Down;
    }

    pub fn turn_left(&mut self) {
        self.current_direction = Direction::Left;
    }

    pub fn turn_right(&mut self) {
        self.current_direction = Direction::Right;
    }

    pub fn custom(current_direction: Unit<Vector3<f32>>) -> Self {
        CurrentDirection {
            current_direction: Direction::Custom(current_direction),
        }
    }
}

impl Default for CurrentDirection {
    fn default() -> Self {
        CurrentDirection {
            current_direction: Direction::default(),
        }
    }
}

impl Component for CurrentDirection {
    type Storage = VecStorage<Self>;
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Custom(Unit<Vector3<f32>>),
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Up
    }
}
