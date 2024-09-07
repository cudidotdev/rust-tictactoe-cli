#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;

fn main() {
    let mut board = [' '; 9];

    let players = ['X', 'O'];

    let mut turn = 0;

    print_board(board);

    loop {
        println!("Enter position for {}", players[turn]);

        let index = get_index_from_input();

        if let Err(reason) = index {
            println!("{reason}");
            continue;
        }

        let index = index.unwrap();

        if let None = index {
            break;
        }

        let index = index.unwrap();

        if board[index] != ' ' {
            println!("The cell at position {} is already occupied", index + 1);
            continue;
        }

        board[index] = players[turn];

        turn = (turn + 1) % 2;

        print_board(board);
    }
}

fn print_board(board: [char; 9]) {
    println!(
        "
  +---+---+---+
  | {} | {} | {} |
  +---+---+---+
  | {} | {} | {} |
  +---+---+---+
  | {} | {} | {} |
  +---+---+---+
  ",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}

fn get_index_from_input() -> Result<Option<usize>, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let input = input.trim();

    if input == "quit" {
        return Ok(None);
    }

    let input = input
        .parse::<usize>()
        .map_err(|_| "Your input should be a positive number".to_string())?;

    if input < 1 || input > 9 {
        return Err(format!("The position should be a number from 1 to 9"));
    }

    Ok(Some(input - 1))
}
