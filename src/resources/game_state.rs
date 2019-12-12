pub enum GameState {
    Gameplay,
    Pause,
    MainMenu,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Gameplay
    }
}

pub struct CurrentState {
    pub current_state: GameState,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState {
            current_state: GameState::Pause,
        }
    }
}

impl CurrentState {
    pub fn pause(&mut self) {
        self.current_state = GameState::Pause;
    }

    pub fn resume(&mut self) {
        self.current_state = GameState::Gameplay;
    }

    pub fn is_paused(&self) -> bool {
        if let GameState::Pause = self.current_state {
            return true;
        }

        false
    }
}
