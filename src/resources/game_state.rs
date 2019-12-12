pub enum GameState {
    Gameplay,
    Pause,
    MainMenu,
    Gameover,
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
            current_state: GameState::MainMenu,
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

    pub fn gameover(&mut self) {
        self.current_state = GameState::Gameover;
    }

    pub fn is_gameplay(&self) -> bool {
        if let GameState::Gameplay = self.current_state {
            return true;
        }

        false
    }

    pub fn is_gameover(&self) -> bool {
        if let GameState::Gameover = self.current_state {
            return true;
        }

        false
    }
}
