use std::io::{self, Write};

use crate::models::{Player, Tabs};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub fn choose_player() -> io::Result<(Player, bool)> {
    let mut stdout = io::stdout();

    let mut tabs = Tabs::new(vec![(17, 13, Player::X), (22, 13, Player::O)]);

    loop {
        //clear screen
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::Purge),
            SetForegroundColor(Color::Cyan),
        )?;

        print_screen();

        // highlight selected tab
        execute!(
            stdout,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
            SetBackgroundColor(Color::Red),
            Print(tabs.value().char()),
            ResetColor,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
        )?;

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => tabs.next(),

                KeyCode::BackTab => tabs.prev(),

                KeyCode::Enter => return Ok((tabs.value().clone(), true)),

                KeyCode::Esc => return Ok((tabs.value().clone(), false)),

                _ => continue,
            }
        }
    }
}

fn print_screen() {
    println!(
        "
     \r    +-------- TIC TAC TOE ---------+
     \r    |                              |
     \r    |    USE TAB TO MOVE CURSOR    |
     \r    |                              |
     \r    |       ENTER TO SELECT        |
     \r    |                              |
     \r    |                              |
     \r    |  +-----------------------+   |
     \r    |  |  CHOOSE YOUR PLAYER   |   |
     \r    |  +-----------------------+   |
     \r    |                              |
     \r    |                              |
     \r    |           <X>  <O>           |
     \r    |                              |
     \r    |                              |
     \r    |  PRESS <ESC> TO QUIT GAME    |
     \r    |                              |
     \r    +------------------------------+
     \n\r"
    );
}
