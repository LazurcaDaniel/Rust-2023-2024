use std::io;

use crossterm::event::{Event, KeyCode, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::{
    event::{read, EnableBracketedPaste},
    execute, queue,
    terminal::enable_raw_mode,
};
use rand::Rng;

const SIZE: usize = 4;
fn move_left(board: &mut [[u16; SIZE]; SIZE]) -> bool {
    let initial_board = board.clone();

    for row in 0..SIZE {
        // Compress non-zero values to the left
        let mut compressed_row: Vec<u16> = board[row]
            .iter()
            .filter(|&&value| value != 0)
            .cloned()
            .collect();

        // Pad the row with zeros to the right
        compressed_row.resize(SIZE, 0);

        // Update the row in the game board
        board[row] = compressed_row.as_slice().try_into().unwrap();
    }

    // Merge adjacent equal values in each row
    for row in 0..SIZE {
        for col in 0..SIZE - 1 {
            if board[row][col] == board[row][col + 1] {
                board[row][col] *= 2;
                board[row][col + 1] = 0;
            }
        }
    }

    // Compress non-zero values to the left again after merging
    for row in 0..SIZE {
        let mut compressed_row: Vec<u16> = board[row]
            .iter()
            .filter(|&&value| value != 0)
            .cloned()
            .collect();

        compressed_row.resize(SIZE, 0);

        board[row] = compressed_row.as_slice().try_into().unwrap();
    }
    for i in 0..4 {
        for j in 0..4 {
            if initial_board[i][j] != board[i][j] {
                return true;
            }
        }
    }
    return false;
}

//fn move_right()

fn is_game_finished(matrix: [[u16; 4]; 4]) -> bool {
    for i in matrix.iter() {
        for &j in i.iter() {
            if j == 2048 {
                return true;
            }
        }
    }
    false
}

fn put_random_value(matrix: &mut [[u16; 4]; 4]) -> [[u16; 4]; 4] {
    let mut rng = rand::thread_rng();
    let mut rand_row: u8 = rng.gen_range(0..4);
    let mut rand_col: u8 = rng.gen_range(0..4);
    while matrix[rand_row as usize][rand_col as usize] != 0 {
        rand_row = rng.gen_range(0..4);
        rand_col = rng.gen_range(0..4);
    }
    let rand_nr = rng.gen_range(0..4);
    if rand_nr == 0 {
        //25% chance of a 4
        matrix[rand_row as usize][rand_col as usize] = 4;
    } else {
        matrix[rand_row as usize][rand_col as usize] = 2
    }
    *matrix
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let supports_keyboard_enhancement = matches!(
        crossterm::terminal::supports_keyboard_enhancement(),
        Ok(true)
    );

    if supports_keyboard_enhancement {
        queue!(
            stdout,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )?;
    }

    execute!(stdout, EnableBracketedPaste,)?;
    let mut game_matrix: [[u16; 4]; 4] = [[0; 4]; 4];
    game_matrix = put_random_value(&mut game_matrix);
    game_matrix = put_random_value(&mut game_matrix);
    for i in 0..4 {
        for j in 0..4 {
            print! {"{} ", game_matrix[i][j]};
        }
        println! {""};
    }
    while !is_game_finished(game_matrix) {
        let event = read()?;

        let mut has_board_changed = false;
        if event == Event::Key(KeyCode::Down.into()) {
            println!("DOWN!");
        } else if event == Event::Key(KeyCode::Up.into()) {
            println!("UP!");
        } else if event == Event::Key(KeyCode::Left.into()) {
            has_board_changed = move_left(&mut game_matrix);
        } else if event == Event::Key(KeyCode::Right.into()) {
            println!("RIGHT!");
        } else if event == Event::Key(KeyCode::Enter.into()) {
            break;
        } else {
            continue;
        }
        for i in 0..4 {
            for j in 0..4 {
                print! {"{} ", game_matrix[i][j]};
            }
            println! {""};
        }
        println! {""};
        if has_board_changed == true {
            game_matrix = put_random_value(&mut game_matrix);
        }
    }

    Ok(())
}
