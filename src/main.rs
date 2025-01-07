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
    // Create a new board
    let board = Board::new(5, 5);
    let mut piece_l = Piece::new('L', vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]);
    let mut piece_u = Piece::new('U', vec![(0, 0), (0, 1), (1, 1), (2, 1), (2, 0)]);
    piece_l.rotate_clockwise();
    piece_u.rotate_counterclockwise();
    board.display(); // Should start empty.
    let valid_boards = find_all_valid_boards(&board, &mut [piece_l, piece_u]);

    println!("Found {} valid boards:", valid_boards.len());
    for (i, valid_board) in valid_boards.iter().enumerate() {
        println!("Board {}:", i + 1);
        valid_board.display();
        println!(); // Add a blank line between boards
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

/// A struct to store valid placements of a piece.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Placement {
    position: (usize, usize), // Position on the board
    rotation: Rotation,       // Rotation of the piece
    symbol: char,             // Symbol for the piece
}

/// Finds all valid placements and returns a vector of boards representing each placement.
pub fn find_all_valid_boards(board: &Board, pieces: &mut [Piece]) -> Vec<Board> {
    let mut valid_boards: Vec<Board> = Vec::new(); // Explicitly define type as Vec<Board>

    for piece in pieces.iter_mut() {
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
    }

    valid_boards
}
