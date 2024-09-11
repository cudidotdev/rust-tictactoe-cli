#[derive(Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }

    pub fn other(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
