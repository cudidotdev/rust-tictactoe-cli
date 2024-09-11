use crate::models::Player;

use super::get_winner;

pub fn get_best_move(board: &[char; 9], computer_player: &Player) -> usize {
    let mut best_score = i32::MIN;
    let mut move_index = 0;

    let mut board = *board;

    // get best score through the minimax algorithm
    for i in 0..9 {
        // if cell is empty, compute the score for that cell
        if board[i] == ' ' {
            board[i] = computer_player.char();

            let score = minimax(&mut board, computer_player, false);

            board[i] = ' ';

            if score > best_score {
                best_score = score;
                move_index = i;
            }
        }
    }

    move_index
}

fn minimax(board: &mut [char; 9], computer_player: &Player, is_computers_turn: bool) -> i32 {
    let human_player = computer_player.other();

    let winner = get_winner(&board);

    // if computer wins
    if winner == Some(computer_player.clone()) {
        return 1;
    }

    // if human wins
    if winner == Some(human_player.clone()) {
        return -1;
    }

    //if draw
    if !board.contains(&' ') {
        return 0;
    }

    if is_computers_turn {
        let mut best_score = i32::MIN;

        for i in 0..9 {
            if board[i] == ' ' {
                board[i] = computer_player.char();

                let score = minimax(board, computer_player, false);

                board[i] = ' ';

                best_score = best_score.max(score);
            }
        }

        best_score
    } else {
        let mut best_score = i32::MAX;

        for i in 0..9 {
            if board[i] == ' ' {
                board[i] = human_player.char();

                let score = minimax(board, computer_player, true);

                board[i] = ' ';

                best_score = best_score.min(score);
            }
        }

        best_score
    }
}
