use std::io::{self, Write};
use std::fs::{File, OpenOptions};

use crossterm::event::{Event, KeyCode, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::{
    event::{read, EnableBracketedPaste},
    execute, queue,
    terminal::enable_raw_mode,
};
use rand::Rng;

const SIZE: usize = 4;
const FILE_NAME: &str = "save_file.txt";

fn move_right(board: &mut [[u16; SIZE]; SIZE]) -> bool {
    let initial_board = board.clone();

    for row in 0..SIZE {
        // Compress non-zero values to the left
        let mut compressed_row: Vec<u16> = board[row]
            .iter()
            .filter(|&&value| value != 0)
            .cloned()
            .collect();

        let mut remaining_zeros = SIZE - compressed_row.len();
        //Pad the vector with zeros to the left
        while remaining_zeros > 0 {
            compressed_row.insert(0, 0);
            if remaining_zeros > 0 {
                remaining_zeros -= 1;
            }
        }
        board[row] = compressed_row.as_slice().try_into().unwrap();
    }

    for row in 0..SIZE {
        for col in (1..4).rev() {
            // Merge adjacent values in the board
            if board[row][col] == board[row][col - 1] {
                board[row][col] *= 2;
                board[row][col - 1] = 0;
            }
        }
    }

    // Recompress all non-zero values to the right
    for row in 0..SIZE {
        let mut compressed_row: Vec<u16> = board[row]
            .iter()
            .filter(|&&value| value != 0)
            .cloned()
            .collect();

        let mut remaining_zeros = SIZE - compressed_row.len();

        while remaining_zeros > 0 {
            compressed_row.insert(0, 0);
            remaining_zeros -= 1;
        }
        board[row] = compressed_row.as_slice().try_into().unwrap();
    }

    //Check of the board has changed to see whether or not to add a new value to the board
    for i in 0..4 {
        for j in 0..4 {
            if initial_board[i][j] != board[i][j] {
                return true;
            }
        }
    }

    return false;
}

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

    //Check of the board has changed to see whether or not to add a new value to the board
    for i in 0..4 {
        for j in 0..4 {
            if initial_board[i][j] != board[i][j] {
                return true;
            }
        }
    }
    return false;
}

fn move_up(board: &mut [[u16; SIZE]; SIZE]) -> bool {
    let initial_board = board.clone();

    for col in 0..4 {
        //compress all non zero values up (or left, as it looks) and let zeroes be down(or right as it looks)
        let mut compressed_row: Vec<u16> = vec![0, 0, 0, 0];
        let mut cnt = 0;
        for row in 0..SIZE {
            if board[row][col] != 0 {
                compressed_row[cnt] = board[row][col];
                cnt += 1;
            }
        }
        //update the values of the column
        for i in 0..SIZE - 1 {
            if compressed_row[i] == compressed_row[i + 1] {
                compressed_row[i] *= 2;
                compressed_row[i + 1] = 0;
            }
        }
        //add the values back to the board
        cnt = 0;

        for i in 0..SIZE {
            if compressed_row[i] != 0 {
                board[cnt][col] = compressed_row[i];
                cnt += 1;
            }
        }
        //fill with zeroes
        for i in cnt..SIZE {
            board[i][col] = 0;
        }
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

fn move_down(board: &mut [[u16; SIZE]; SIZE]) -> bool {
    let initial_board = board.clone();
    for col in 0..SIZE {
        //compress all non zero values up (or left, as it looks) and let zeroes be down(or right as it looks)
        let mut compressed_row: Vec<u16> = vec![0, 0, 0, 0];
        let mut cnt = 0;
        for row in (0..SIZE).rev() {
            if board[row][col] != 0 {
                compressed_row[cnt] = board[row][col];
                cnt += 1;
            }
        }
        //update the values of the column
        for i in 0..SIZE - 1 {
            if compressed_row[i] == compressed_row[i + 1] {
                compressed_row[i] *= 2;
                compressed_row[i + 1] = 0;
            }
        }
        //add the values back to the board
        cnt = SIZE - 1;

        for i in 0..SIZE {
            if compressed_row[i] != 0 {
                board[cnt][col] = compressed_row[i];
                if cnt > 0 {
                    cnt -= 1;
                }
            }
        }
        //fill with zeroes
        for i in 0..=cnt {
            board[i][col] = 0;
        }
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

fn save_game(board: &[[u16;SIZE];SIZE]) -> io::Result<()>
{
    match OpenOptions::new().write(true).truncate(true).create(true).open(FILE_NAME)
    {
        Ok(mut file) =>
        {
            for row in board
            {
                for &value in row
                {
                    write!(file, "{} ", value)?;
                }
                writeln!(file)?;
            }
            Ok(())
        }
        Err(error) =>
        {
            return Err(error);
        }
    }
}



fn play_game(new_game: bool) -> io::Result<bool>
{
    let mut game_matrix: [[u16; 4]; 4]= [[0; 4]; 4];
    if new_game {
    game_matrix = put_random_value(&mut game_matrix);
    game_matrix = put_random_value(&mut game_matrix);
    }

    for i in 0..4 {
        for j in 0..4 {
            print! {"{} ", game_matrix[i][j]};
        }
        println!("");
    }
    println!("");

    while !is_game_finished(game_matrix) {
        let event = read()?;

        if event == Event::Key(KeyCode::Down.into()) {
            if move_down(&mut game_matrix) {
                game_matrix = put_random_value(&mut game_matrix);
            }
        } else if event == Event::Key(KeyCode::Up.into()) {
            if move_up(&mut game_matrix) {
                game_matrix = put_random_value(&mut game_matrix);
            }
        } else if event == Event::Key(KeyCode::Left.into()) {
            if move_left(&mut game_matrix) {
                game_matrix = put_random_value(&mut game_matrix);
            }
        } else if event == Event::Key(KeyCode::Right.into()) {
            if move_right(&mut game_matrix) {
                game_matrix = put_random_value(&mut game_matrix);
            }
        } else if event == Event::Key(KeyCode::Enter.into()) {
            break;
        } else {
            continue;
        }
        save_game(&game_matrix)?;
        for i in 0..4 {
            for j in 0..4 {
                print! {"{} ", game_matrix[i][j]};
            }
            println!("");
        }
        println!("");
    }
    Ok(true)
}

fn how_to_play()
{
    println!("Use your arrow keys or swipe left or right, up or down to move the tiles.
When two tiles with the same number touch, 
they merge into one. Once you get 2048 in any square, you win. Once you reach a state
where no more possible moves can be made, you lose!");
    println!("The game saves automatically after every move so you can pick up right where you left from!");
}

fn menu() -> io::Result<bool> {
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

    println!("Welcome to 2048!");
    println!("Choose an option by typing the number coresponding to it.");
    println!("1. New game");
    println!("2. Load game");
    println!("3. How to play");
    println!("4. Quit");
    loop {
        match read() {
            Ok(event) => {
                if event == Event::Key(KeyCode::Char('1').into()) {
                    match play_game(true)
                    {
                        Ok(state) =>
                        {   
                            return Ok(state);
                        }
                        Err(error) =>
                        {
                            return Err(error);
                        }
                    }
                } else if event == Event::Key(KeyCode::Char('2').into()) {
                    match play_game(false)
                    {
                        Ok(state) =>
                        {
                            return Ok(state);
                        }
                        Err(error) =>
                        {
                            return Err(error);
                        }
                    }
                } else if event == Event::Key(KeyCode::Char('3').into()) {
                    how_to_play();
                } else if event == Event::Key(KeyCode::Char('4').into()) {
                    break;
                } else {
                    continue;
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    Ok(true)
}

fn main() -> io::Result<()> {
    match menu() {
        Ok(_) => {
            println!("Goodbye! See you around!");
        }
        Err(error) => {
            return Err(error);
        }
    }
    

    Ok(())
}
