use std::io::{self, Write};

use crate::models::{Player, Tabs};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub fn end_menu(
    winning_player: &Option<Player>,
    human_player: &Player,
    board: &[char; 9],
) -> io::Result<bool> {
    let mut stdout = io::stdout();

    let mut tabs = Tabs::new(vec![(16, 15, ("RESTART", true)), (17, 17, ("QUIT", false))]);

    loop {
        //clear screen
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::Purge),
            SetForegroundColor(Color::Cyan),
        )?;

        print_screen(&board);

        let (statement, color) = match winning_player {
            Some(winner) => {
                if winner == human_player {
                    ("YOU WIN!!", Color::Green)
                } else {
                    ("YOU LOSE!!", Color::Magenta)
                }
            }
            None => ("IT'S A TIE!", Color::Yellow),
        };

        execute!(
            stdout,
            SetForegroundColor(color),
            cursor::MoveTo(7, 3),
            Print("+-----------------------+"),
            cursor::MoveTo(7, 4),
            Print("|                       |"),
            cursor::MoveTo(7, 5),
            Print("+-----------------------+"),
            cursor::MoveTo(14, 4),
            Print(statement),
            ResetColor,
        )?;

        // highlight selected tab
        execute!(
            stdout,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
            SetBackgroundColor(Color::Red),
            Print(tabs.value().0),
            ResetColor,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
        )?;

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => tabs.next(),

                KeyCode::BackTab => tabs.prev(),

                KeyCode::Enter => return Ok(tabs.value().1),

                KeyCode::Esc => return Ok(false),

                _ => continue,
            }
        }
    }
}

fn print_screen(board: &[char; 9]) {
    println!(
        "
     \r    +-------- TIC TAC TOE ---------+
     \r    |                              |
     \r    |  +-----------------------+   |
     \r    |  |                       |   |
     \r    |  +-----------------------+   |
     \r    |                              |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |                              |
     \r    |          <RESTART>           |
     \r    |                              |
     \r    |           <QUIT>             |
     \r    |                              |
     \r    +------------------------------+
     \n\r",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}
