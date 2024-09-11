use std::io::{self, Write};

use crate::{
    models::{Player, Tabs},
    utils::{get_best_move, get_winner},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub fn game_play(human_player: &Player) -> io::Result<(Option<Player>, [char; 9], bool)> {
    let mut stdout = io::stdout();

    let mut board = [' '; 9];

    let mut tabs = Tabs::<usize>::new(vec![
        (15, 9, 0),
        (19, 9, 1),
        (23, 9, 2),
        (15, 11, 3),
        (19, 11, 4),
        (23, 11, 5),
        (15, 13, 6),
        (19, 13, 7),
        (23, 13, 8),
    ]);

    let computer_player = human_player.other();

    let mut turn = Player::X;

    loop {
        //clear screen
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::Purge),
            SetForegroundColor(Color::Cyan),
        )?;

        print_screen(board);

        // highlight selected tab
        execute!(
            stdout,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
            SetBackgroundColor(Color::Red),
            Print(board[*tabs.value()]),
            ResetColor,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
        )?;

        stdout.flush()?;

        // check if the game is won
        let winner = get_winner(&board);

        if let Some(winner) = winner {
            return Ok((Some(winner), board, true));
        }

        //if draw
        if !board.contains(&' ') {
            return Ok((None, board, true));
        }

        // play if its the computers turn
        if turn == computer_player {
            let best_move = get_best_move(&board, &computer_player);

            board[best_move] = computer_player.char();

            turn = turn.other();

            loop {
                tabs.next();

                // if there's no empty cell or the current cell is empty terminate the loop
                if !board.contains(&' ') || board[*tabs.value()] == ' ' {
                    break;
                }
            }

            continue;
        }

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => loop {
                    tabs.next();

                    if board[*tabs.value()] == ' ' {
                        break;
                    }
                },

                KeyCode::BackTab => loop {
                    tabs.prev();

                    if board[*tabs.value()] == ' ' {
                        break;
                    }
                },

                KeyCode::Enter => {
                    board[*tabs.value()] = human_player.char();

                    turn = turn.other();
                }

                KeyCode::Esc => return Ok((None, board, false)),

                _ => continue,
            }
        }
    }
}

fn print_screen(board: [char; 9]) {
    println!(
        "
     \r    +-------- TIC TAC TOE ---------+
     \r    |                              |
     \r    |    USE TAB TO MOVE CURSOR    |
     \r    |                              |
     \r    |       ENTER TO SELECT        |
     \r    |                              |
     \r    |                              |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |                              |
     \r    |                              |
     \r    |  PRESS <ESC> TO QUIT GAME    |
     \r    |                              |
     \r    +------------------------------+
     \n\r",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}
