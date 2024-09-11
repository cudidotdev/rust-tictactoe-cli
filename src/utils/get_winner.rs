use crate::models::Player;

pub fn get_winner(board: &[char; 9]) -> Option<Player> {
    // Define the winning combinations
    let winning_combinations: [[usize; 3]; 8] = [
        [0, 1, 2], // Top row
        [3, 4, 5], // Middle row
        [6, 7, 8], // Bottom row
        [0, 3, 6], // Left column
        [1, 4, 7], // Middle column
        [2, 5, 8], // Right column
        [0, 4, 8], // Top-left to bottom-right diagonal
        [2, 4, 6], // Top-right to bottom-left diagonal
    ];

    let x = Player::X.char();

    let o = Player::O.char();

    for combo in winning_combinations {
        let [a, b, c] = combo;

        if board[a] == x && board[b] == x && board[c] == x {
            return Some(Player::X);
        }

        if board[a] == o && board[b] == o && board[c] == o {
            return Some(Player::O);
        }
    }

    None
}
