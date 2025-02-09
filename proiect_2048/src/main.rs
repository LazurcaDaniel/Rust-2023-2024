use std::fs::{File, OpenOptions};

use std::io::{self, BufRead, BufReader, Write, ErrorKind, Error};

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
    let initial_board = *board;

    for row in board.iter_mut().take(SIZE) {
        // Compress non-zero values to the left
        let compressed_row: Vec<u16> =
            row.iter().filter(|&&value| value != 0).cloned().collect();
       
        let remaining_zeros = SIZE - compressed_row.len();
        let mut compressed_row = compressed_row;
        compressed_row.extend(std::iter::repeat(0).take(remaining_zeros));
       
        // Update the row with the compressed values
            for (i, value) in compressed_row.iter().rev().enumerate() {
                row[i] = *value;
            }
    }

    for row in board.iter_mut().take(SIZE) {
        for col in (1..SIZE).rev() {
            // Merge adjacent values in the board
            if row[col] == row[col - 1] {
                row[col] *= 2;
                row[col - 1] = 0;
            }
        }
    }

    // Recompress all non-zero values to the right
    for row in board.iter_mut().take(SIZE) {
        let compressed_row: Vec<u16> =
            row.iter().filter(|&&value| value != 0).cloned().collect();

        let remaining_zeros = SIZE - compressed_row.len();

        let mut compressed_row = compressed_row;
        compressed_row.extend(std::iter::repeat(0).take(remaining_zeros));

        // Update the row with the compressed values
        for (i, value) in compressed_row.iter().rev().enumerate() {
            row[i] = *value;
        }
    }

    //Check of the board has changed to see whether or not to add a new value to the board
    for i in 0..4 {
        for j in 0..4 {
            if initial_board[i][j] != board[i][j] {
                return true;
            }
        }
    }

    false
}

fn move_left(board: &mut [[u16; SIZE]; SIZE]) -> bool {
    let initial_board = *board;

    for row in board.iter_mut().take(SIZE) {
        // Compress non-zero values to the left
        let compressed_row: Vec<u16> = row.iter().filter(|&&value| value != 0).cloned().collect();

        // Pad the row with zeros to the right
        let mut compressed_row = compressed_row;
        compressed_row.resize(SIZE, 0);

        // Update the row in the game board
        for (i, value) in compressed_row.iter().enumerate() {
            row[i] = *value;
        }
    }

    // Merge adjacent equal values in each row
    for row in board.iter_mut().take(SIZE) {
        for col in 0..SIZE - 1 {
            if row[col] == row[col + 1] {
                row[col] *= 2;
                row[col + 1] = 0;
            }
        }
    }

    // Compress non-zero values to the left again after merging
    for row in board.iter_mut().take(SIZE) {
        let compressed_row: Vec<u16> = row.iter().filter(|&&value| value != 0).cloned().collect();

        let mut compressed_row = compressed_row;
        compressed_row.resize(SIZE, 0);

        for (i, value) in compressed_row.iter().enumerate() {
            row[i] = *value;
        }
    }

    //Check of the board has changed to see whether or not to add a new value to the board
    for i in 0..4 {
        for j in 0..4 {
            if initial_board[i][j] != board[i][j] {
                return true;
            }
        }
    }
    false
}

fn move_up(board: &mut [[u16; SIZE]; SIZE]) -> bool {
    let initial_board = *board;

    for col in 0..4 {
        //compress all non zero values up (or left, as it looks) and let zeroes be down(or right as it looks)
        let mut compressed_row: Vec<u16> = vec![0, 0, 0, 0];
        let mut cnt = 0;
        for row in board.iter_mut().take(SIZE) {
            if row[col] != 0 {
                compressed_row[cnt] = row[col];
                cnt += 1;
            }
        }

        //update the values of the column
        let mut iter = compressed_row.iter_mut().take(SIZE).peekable();
        
        while let Some(i) = iter.next() {
            if let Some(next_value) = iter.peek() {
                if *i == **next_value {
                    *i *= 2;
                    *iter.next().unwrap() = 0;
                }
            }
        }
        //add the values back to the board
        cnt = 0;
        for &value in compressed_row[..SIZE].iter() {
            if value != 0 {
                board[cnt][col] = value;
                cnt += 1;
            }
        }
        //fill with zeroes
        for row in board.iter_mut().take(SIZE).skip(cnt) {
            row[col] = 0;
        }
    }
    for i in 0..4 {
        for j in 0..4 {
            if initial_board[i][j] != board[i][j] {
                return true;
            }
        }
    }
    false
}

fn move_down(board: &mut [[u16; SIZE]; SIZE]) -> bool {
    let initial_board = *board;
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
        compressed_row =
            compressed_row.iter().filter(|&&value| value != 0).cloned().collect();
        
        //add the values back to the board
        cnt = SIZE;

        for &value in compressed_row.iter() {
                board[cnt-1][col] = value;
                cnt = cnt.saturating_sub(1);
        }

        
        //fill with zeroes
        
        for row in board.iter_mut().take(cnt) {
            row[col] = 0;
        }
    }
    for i in 0..4 {
        for j in 0..4 {
            if initial_board[i][j] != board[i][j] {
                return true;
            }
        }
    }
    false
}

fn is_game_finished(matrix: [[u16; 4]; 4]) -> u8 {
    let mut dummy_matrix = matrix;

    let mut loser_cnt = 0;
    if !move_down(&mut dummy_matrix) {
        loser_cnt += 1;
    }
    dummy_matrix = matrix;
    if !move_up(&mut dummy_matrix) {
        loser_cnt += 1;
    }

    dummy_matrix = matrix;
    if !move_left(&mut dummy_matrix) {
        loser_cnt += 1;
    }

    dummy_matrix = matrix;
    if !move_right(&mut dummy_matrix) {
        loser_cnt += 1;
    }

    if loser_cnt == 4 {
        return 2;
    }
    for i in matrix.iter() {
        for &j in i.iter() {
            if j == 2048 {
                return 1;
            }
        }
    }

    0
}

fn put_random_value(matrix: &mut [[u16; 4]; 4]) -> [[u16; 4]; 4] {
    let mut ok: bool = false;
    for row in matrix.iter_mut().take(SIZE) {
        for &val in row.iter().take(SIZE) {
            if val == 0 {
                ok = true;
                break;
            }
        }
    }
    if !ok {
        return *matrix;
    }

    let mut rng = rand::thread_rng();
    let mut rand_row: u8 = rng.gen_range(0..4);
    let mut rand_col: u8 = rng.gen_range(0..4);
    while matrix[rand_row as usize][rand_col as usize] != 0 {
        rand_row = rng.gen_range(0..4);
        rand_col = rng.gen_range(0..4);
    }
    let rand_nr = rng.gen_range(0..20);
    if rand_nr == 0 {
        //5% chance of a 4
        matrix[rand_row as usize][rand_col as usize] = 4;
    } else {
        matrix[rand_row as usize][rand_col as usize] = 2
    }
    
    *matrix
}

fn save_game(board: &[[u16; SIZE]; SIZE]) -> io::Result<()> {
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE_NAME)
    {
        Ok(mut file) => {
            for row in board {
                for &value in row {
                    write!(file, "{} ", value)?;
                }
                writeln!(file)?;
            }
            Ok(())
        }
        Err(error) => Err(error)  
    }
}

fn load_game() -> io::Result<[[u16; SIZE]; SIZE]> {
    let file = File::open(FILE_NAME)?;

    let mut game_matrix: [[u16; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    // Create BufReader outside the loop
    let mut buf_reader = BufReader::new(file);
    
    
    for i in 0..SIZE {
        let mut buffer = String::new();

        let bytes = buf_reader.read_line(&mut buffer)?;
        // Read a line from the buffered reader
        match bytes.cmp(&0) {
            std::cmp::Ordering::Greater => {
                // Parse values and fill the matrix
                if bytes < SIZE * std::mem::size_of::<u16>()
                {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "File does not contain enough data for the matrix",
                    ));
                }
                
                for (j, value_str) in buffer.split_whitespace().enumerate().take(SIZE) {
                    if let Ok(value) = value_str.parse() {
                        game_matrix[i][j] = value;
                    }
                }
                
            }
            std::cmp::Ordering::Equal => return Ok(game_matrix),
            std::cmp::Ordering::Less => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "File does not contain enough data for the matrix",
                ));
            }
        }
    }
    
    
    Ok(game_matrix)
}

fn is_matrix_empty(matrix: [[u16; SIZE]; SIZE]) -> bool {
    for row in matrix.iter() {
        for &val in row {
            if val != 0 {
                return true;
            }
        }
    }
    false
}

fn clear_file_contents(file_path: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    // Set the length of the file to 0, effectively clearing its contents
    file.set_len(0)?;

    Ok(())
}

fn play_game(new_game: bool) -> io::Result<u8> {
    let mut game_matrix: [[u16; 4]; 4] = [[0; 4]; 4];
    if new_game {
        clear_file_contents(FILE_NAME)?;
        game_matrix = put_random_value(&mut game_matrix);
        game_matrix = put_random_value(&mut game_matrix);
    } else {
        match load_game() {
            Ok(matrix) => {
                if !is_matrix_empty(matrix) {
                    println!("You have no saved game. A new game will be generated for you!");
                    game_matrix = put_random_value(&mut game_matrix);
                    game_matrix = put_random_value(&mut game_matrix);
                } else {
                    game_matrix = matrix;
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    for row in &game_matrix {
        for &val in row.iter().take(SIZE) {
            let spaces = " ".repeat(5 - val.to_string().len());
            print! {"{}{}",val,spaces};
        }
        println!();
    }
    println!();
    let mut state = 0;
    //let mut state = is_game_finished(game_matrix);
    while state == 0 {
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
        } else if event == Event::Key(KeyCode::Esc.into()) {
            break;
        } else {
            continue;
        }
        save_game(&game_matrix)?;
        for row in &game_matrix {
            for &val in row.iter().take(SIZE) {
                let spaces = " ".repeat(5 - val.to_string().len());
                print! {"{}{}",val,spaces};
            }
            println!();
        }
        println!();
        state = is_game_finished(game_matrix);
    }
    Ok(state)
}

fn how_to_play() {
    println!(
        "Use your arrow keys or swipe left or right, up or down to move the tiles.
When two tiles with the same number touch, 
they merge into one. Once you get 2048 in any square, you win. Once you reach a state
where no more possible moves can be made, you lose!"
    );
    println!("The game saves automatically after every move so you can pick up right where you left from!");
    println!("You can press ESC at any time to quit the game.")
}

fn menu() -> io::Result<u8> {
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
                    match play_game(true) {
                        Ok(state) => {
                            return Ok(state);
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                } else if event == Event::Key(KeyCode::Char('2').into()) {
                    match play_game(false) {
                        Ok(state) => {
                            return Ok(state);
                        }
                        Err(error) => {
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
    Ok(3)
}

fn main() -> io::Result<()> {
    match menu() {
        Ok(state) => match state {
            1 => {
                print!("Congratulations! You won!");
                clear_file_contents(FILE_NAME)?;
            }
            2 => {
                print!("Game over!");
                clear_file_contents(FILE_NAME)?;
            }
            _ => print!("Goodbye! See you around!"),
        },
        Err(error) => {
            return Err(error);
        }
    }

    Ok(())
}
