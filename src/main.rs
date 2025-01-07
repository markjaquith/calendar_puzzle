mod board;
mod piece;

use board::Board;
use piece::{Piece, Rotation};

use dialoguer::{theme::ColorfulTheme, Select};

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
    // Define the initial board
    let width = 3;
    let height = 3;
    let mut initial_board = Board::new(width, height, '·');
    initial_board.place_piece(
        &Piece::new('☻', vec![(0, 0)], (255, 255, 255), (37, 59, 37)),
        1,
        1,
    );
    println!("Initial board:");
    initial_board.display();
    println!(); // Blank line between boards

    // Define the pieces to place
    let mut pieces = vec![
        Piece::new(
            '■',
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            (88, 28, 71),
            (245, 224, 220),
        ),
        Piece::new(
            '▲',
            vec![(0, 0), (0, 1), (1, 0)],
            (64, 140, 86),
            (220, 245, 230),
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
    let final_boards = find_all_boards_placing_all_pieces(initial_board, &mut pieces);

    println!(
        "Found {} boards that successfully place all pieces:",
        final_boards.len()
    );

    // Display each valid board
    for (i, board) in final_boards.iter().enumerate() {
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
pub fn find_all_boards_placing_all_pieces(board: Board, pieces: &mut Vec<Piece>) -> Vec<Board> {
    // Base case: If no pieces are left, return the current board
    if pieces.is_empty() {
        return vec![board];
    }

    // Get the first piece and generate all valid boards with it
    let mut piece = pieces.remove(0); // Take ownership of the first piece
    let valid_boards = find_all_valid_boards_with_new_piece(&board, &mut piece);

    // Container to collect all successful boards
    let mut all_boards = Vec::new();

    // For each valid board from placing the current piece, recurse with the remaining pieces
    for valid_board in valid_boards {
        let mut remaining_pieces = pieces.clone(); // Clone the remaining pieces
        let boards = find_all_boards_placing_all_pieces(valid_board, &mut remaining_pieces);
        all_boards.extend(boards); // Add successful boards to the result
    }

    // Put the piece back for the caller (to avoid modifying the original vector)
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
                    valid_boards.push(new_board); // Push the owned board
                }
            }
        }

        // Reset the piece to its original rotation after testing
        piece.reset_rotation();
    }

    valid_boards
}
