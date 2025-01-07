mod board;
mod piece;

use std::io::Write;

use board::Board;
use piece::{Piece, Rotation};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

/// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Stop as soon as the first valid board is found
    #[arg(short, long)]
    first: bool,
}

/// Represents the months of the year.
#[derive(Debug, Clone, Copy)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    /// Converts an index (0-11) to a `Month`.
    fn from_index(index: usize) -> Self {
        match index {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            _ => unreachable!("Invalid month index"),
        }
    }

    /// Returns a string slice of the month's name.
    fn as_str(&self) -> &'static str {
        match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
    }
}

/// Represents the days of the week.
#[derive(Debug, Clone, Copy)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    /// Converts an index (0-6) to a `Weekday`.
    fn from_index(index: usize) -> Self {
        match index {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            6 => Weekday::Sunday,
            _ => unreachable!("Invalid weekday index"),
        }
    }

    /// Returns a string slice of the weekday's name.
    fn as_str(&self) -> &'static str {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }
}

/// A struct representing a specific day.
#[derive(Debug)]
struct Day {
    month: Month,
    day: u8,
    weekday: Weekday,
}

impl Day {
    /// Creates a new `Day`.
    fn new(month: Month, day: u8, weekday: Weekday) -> Self {
        Day {
            month,
            day,
            weekday,
        }
    }

    /// Displays the details of the `Day`.
    fn display(&self) {
        println!(
            "Selected Date:\nMonth: {}\nDay: {}\nDay of Week: {}",
            self.month.as_str(),
            self.day,
            self.weekday.as_str()
        );
    }
}

fn main() {
    let args = Args::parse();
    let first = args.first;

    // Atomic flag for tracking whether a valid board has been found.
    let found = AtomicBool::new(false);

    // Define the initial board
    let width = 9;
    let height = 6;
    let mut initial_board = Board::new(width, height, '·');
    initial_board.place_piece(
        &Piece::new('☻', vec![(0, 0)], (255, 255, 255), (37, 59, 37)),
        8,
        0,
    );
    initial_board.place_piece(
        &Piece::new('☺', vec![(0, 0)], (255, 255, 255), (37, 59, 37)),
        0,
        0,
    );
    initial_board.place_piece(
        &Piece::new('☼', vec![(0, 0)], (255, 255, 255), (37, 59, 37)),
        4,
        1,
    );

    // Corner piece
    initial_board.place_piece(
        &Piece::new('☗', vec![(0, 0)], (255, 255, 255), (37, 59, 37)),
        8,
        5,
    );
    println!("Initial board:");
    initial_board.display();
    println!(); // Blank line between boards

    // Define the pieces to place
    let mut pieces = vec![
        // A A A A A
        Piece::new(
            'A',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
            (64, 140, 86),
            (220, 245, 230),
        ),
        // B B B B
        // B
        Piece::new(
            'B',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)],
            (0, 0, 0),
            (255, 255, 255),
        ),
        // C
        // C C
        // C C
        Piece::new(
            'C',
            vec![(0, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
            (0, 0, 0),
            (255, 255, 255),
        ),
        // D D
        // D
        // D D
        Piece::new(
            'D',
            vec![(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)],
            (128, 128, 64),
            (239, 235, 231),
        ),
        // E
        // E E E
        // E
        Piece::new(
            'E',
            vec![(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)],
            (0, 0, 0),
            (255, 255, 255),
        ),
        // F F F F
        //   F
        Piece::new(
            'F',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (1, 1)],
            (0, 0, 0),
            (255, 255, 255),
        ),
        // G
        // G
        // G G G
        Piece::new(
            'G',
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            (88, 28, 71),
            (245, 224, 220),
        ),
        // H
        // H H H
        //     H
        Piece::new(
            'H',
            vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
            (0, 0, 0),
            (255, 255, 255),
        ),
        //   I I I
        // I I
        Piece::new(
            'I',
            vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1)],
            (0, 0, 0),
            (255, 255, 255),
        ),
        //   J
        // J J
        //   J J
        Piece::new(
            'J',
            vec![(1, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            (0, 0, 0),
            (255, 255, 255),
        ),
    ];

    println!("Pieces to place:");
    for piece in &pieces {
        // Make an example board just big enough to display this piece.
        let mut example_board = Board::new(
            piece.get_dimensions().0 as usize,
            piece.get_dimensions().1 as usize,
            ' ',
        );
        // Place the piece in the top-left corner for display
        example_board.place_piece(piece, 0, 0);
        example_board.display();
        println!(); // Blank line between boards
    }

    // Generate all valid boards that place all pieces
    let final_boards =
        find_all_boards_placing_all_pieces(initial_board, &mut pieces, &found, first);

    println!(
        "Found {} boards that successfully place all pieces:",
        final_boards.len()
    );

    // Display the first 1_000 boards
    let max_boards_to_display = 1_000;
    let should_clear_screen = false;

    for (i, board) in final_boards.iter().take(max_boards_to_display).enumerate() {
        if should_clear_screen {
            clear_screen();
        }
        println!("Board {}:", i + 1);
        board.display();
        println!(); // Blank line between boards
    }

    // Call an unused function to demonstrate the linter
    let call_unused_function = false;
    if call_unused_function {
        unused_function();
    }
}

fn clear_screen() {
    // Clear the terminal screen and move the cursor to the top-left
    print!("\x1b[2J\x1b[H");
    std::io::stdout().flush().unwrap(); // Ensure the screen is cleared immediately
}

fn unused_function() {
    let early_return = true;

    // Return without warning.
    if early_return {
        return;
    }

    // Simulate an interactive selection process
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let weekdays = vec![
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    // Choose a month
    let month_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a month")
        .items(&months)
        .default(0)
        .interact()
        .unwrap();

    let month = Month::from_index(month_index);

    // Choose a day
    let day = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a day of the month")
        .items(&(1..=31).map(|d| d.to_string()).collect::<Vec<_>>())
        .default(0)
        .interact()
        .unwrap() as u8
        + 1;

    // Choose a day of the week
    let weekday_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a day of the week")
        .items(&weekdays)
        .default(0)
        .interact()
        .unwrap();

    let weekday = Weekday::from_index(weekday_index);

    // Create and display the selected day
    let selected_day = Day::new(month, day, weekday);
    selected_day.display();
}

/// Recursively attempts to place all pieces on the board.
/// Returns a vector of boards that successfully place all pieces.
pub fn find_all_boards_placing_all_pieces(
    board: Board,
    pieces: &mut Vec<Piece>,
    found: &AtomicBool,
    first: bool,
) -> Vec<Board> {
    // If no pieces are left, return the current board
    if pieces.is_empty() {
        if first {
            found.store(true, Ordering::Relaxed);
        }
        return vec![board];
    }

    // Remove the first piece and get all valid placements
    let mut piece = pieces.remove(0);
    let valid_boards = find_all_valid_boards_with_new_piece(&board, &mut piece);

    // Use parallel iterator to process valid boards
    let all_boards: Vec<Board> = valid_boards
        .into_par_iter() // Convert to parallel iterator
        .flat_map(|valid_board| {
            if found.load(Ordering::Relaxed) && first {
                return vec![]; // Terminate early if `--first` is set and a board is found
            }
            let mut remaining_pieces = pieces.clone();
            find_all_boards_placing_all_pieces(valid_board, &mut remaining_pieces, found, first)
        })
        .collect();

    // Restore the removed piece for the caller
    pieces.insert(0, piece);

    all_boards
}

/// Finds all valid placements and returns a vector of boards representing each placement.
pub fn find_all_valid_boards_with_new_piece(board: &Board, piece: &mut Piece) -> Vec<Board> {
    let mut valid_boards: Vec<Board> = Vec::new();

    for rotation in [
        Rotation::Zero,
        Rotation::Ninety,
        Rotation::OneEighty,
        Rotation::TwoSeventy,
    ] {
        // Rotate the piece to the current orientation
        while piece.get_rotation() != rotation {
            piece.rotate_clockwise();
        }

        // Try placing the piece in every position on the board
        for y in 0..board.height {
            for x in 0..board.width {
                if board.can_place_piece(piece, x as i32, y as i32).is_ok() {
                    let mut new_board = board.clone(); // Clone the current board
                    new_board.place_piece(piece, x as i32, y as i32); // Place the piece
                    if !new_board.has_dead_end_blanks_smaller_than(5) {
                        valid_boards.push(new_board); // Push the owned board
                    }
                }
            }
        }

        // Reset the piece to its original rotation after testing
        piece.reset_rotation();
    }

    valid_boards
}
