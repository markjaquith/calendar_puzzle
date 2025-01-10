mod board;
mod calendar;
mod piece;
mod pieces;

use board::Board;
use calendar::{Day, Month, MonthDay, Weekday};
use piece::{Piece, Rotation};
use pieces::{get_corner_piece, get_default_pieces};

use chrono::Datelike;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use strum::IntoEnumIterator;

/// Command-line arguments
#[derive(Parser)]
struct Args {
    // Show all solutions
    #[arg(short, long, default_value = "false")]
    all: bool,

    //  Show the pieces to place.
    #[arg(long = "show-pieces")]
    show_pieces: bool,

    #[arg(short, long)]
    today: bool,
}

fn main() {
    let args = Args::parse();
    let all = args.all;
    let first = !all;
    let show_pieces = args.show_pieces;
    let today = args.today;
    let now = chrono::Local::now();

    let mut day = Day::new(
        Month::from_str(&now.format("%B").to_string()).unwrap(),
        MonthDay::new(now.day() as u8).unwrap(),
        Weekday::from_str(&now.format("%A").to_string()).unwrap(),
    );

    if !today {
        day = select_day();
    }

    // Atomic flag for tracking whether a valid board has been found.
    let found = AtomicBool::new(false);

    // Corner coordinates
    let corner_coordinates = (8, 5);

    // Define the initial board
    let width = 9;
    let height = 6;
    let mut initial_board = Board::new(width, height, '·');
    initial_board.place_piece(
        &Piece::new('☻', vec![(0, 0)], (255, 255, 255), (0, 0, 0)),
        day.month.to_coordinates(),
    );
    initial_board.place_piece(
        &Piece::new('◉', vec![(0, 0)], (255, 255, 255), (0, 0, 0)),
        day.day.to_coordinates(),
    );
    initial_board.place_piece(
        &Piece::new('☼', vec![(0, 0)], (255, 255, 255), (0, 0, 0)),
        day.weekday.to_coordinates(),
    );

    // Corner piece
    initial_board.place_piece(&get_corner_piece(), corner_coordinates);

    println!("{}, {} {}", day.weekday, day.month, day.day);
    initial_board.display();
    println!();

    // Define the pieces to place
    let mut pieces = get_default_pieces();

    if show_pieces {
        println!("Pieces to place:");
        for piece in &pieces {
            // Make an example board just big enough to display this piece.
            let mut example_board = Board::new(
                piece.get_dimensions().0 as usize,
                piece.get_dimensions().1 as usize,
                ' ',
            );
            // Place the piece in the top-left corner for display
            example_board.place_piece(piece, (0, 0));
            example_board.display();
            println!();
        }
    }

    // Generate all valid boards that place all pieces
    let final_boards =
        find_all_boards_placing_all_pieces(initial_board, &mut pieces, &found, first);

    if !first && final_boards.len() > 1 {
        println!(
            "Found {} boards that successfully place all pieces:",
            final_boards.len()
        );
    }

    // Display the first 1_000 boards
    let max_boards_to_display = 1_000;

    for (i, board) in final_boards.iter().take(max_boards_to_display).enumerate() {
        if all {
            println!("Solution {}:", i + 1);
        }

        board.display();
        println!();
    }
}

/// Gets input from the user to create a `Day` struct
fn select_day() -> Day {
    // Choose a month
    let months = Month::iter().map(|m| m.to_string()).collect::<Vec<_>>();
    let month_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a month")
        .items(&months)
        .default(0)
        .interact()
        .unwrap();
    let month = Month::from_str(&months[month_index]).unwrap();

    let day_count = month.day_count();

    // Choose a month day
    let month_day_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a day of the month")
        .items(&(1..=day_count).map(|d| d.to_string()).collect::<Vec<_>>())
        .default(0)
        .interact()
        .unwrap();

    let day = MonthDay::new((month_day_index + 1) as u8).expect("Invalid day of the month");
    let _ = day.is_valid_in_month(&month) || panic!("Invalid day/month combo");

    // Choose a day of the week
    let weekdays = Weekday::iter().map(|w| w.to_string()).collect::<Vec<_>>();
    let weekday_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a day of the week")
        .items(&weekdays)
        .default(0)
        .interact()
        .unwrap();
    let weekday = Weekday::from_str(&weekdays[weekday_index]).unwrap();

    Day::new(month, day, weekday)
}

/// Recursively attempts to place all pieces on the board.
/// Returns a vector of boards that successfully place all pieces.
pub fn find_all_boards_placing_all_pieces(
    board: Board,
    pieces: &mut Vec<Piece>,
    found: &AtomicBool,
    first: bool,
) -> HashSet<Board> {
    // If no pieces are left, return the current board
    if pieces.is_empty() {
        // If the `--first` flag is set and a board has been found, mark it as found so other
        // threads can terminate early.
        if first {
            found.store(true, Ordering::Relaxed);
        }
        let mut final_board = HashSet::new();
        final_board.insert(board);
        return final_board;
    }

    // Remove the first piece and get all valid placements
    let mut piece = pieces.remove(0);
    let valid_boards = find_all_valid_boards_with_new_piece(&board, &mut piece);

    // Use parallel iterator to process the valid boards
    let all_boards: HashSet<Board> = valid_boards
        .into_par_iter() // Convert to parallel iterator
        .flat_map(|valid_board| {
            if first && found.load(Ordering::Relaxed) {
                return HashSet::new(); // Terminate early if `--first` is set and a board is found
            }
            let mut remaining_pieces = pieces.clone();
            find_all_boards_placing_all_pieces(valid_board, &mut remaining_pieces, found, first)
                .into_iter() // Convert the returned Vec<Board> into an iterator
                .collect::<HashSet<_>>() // Collect into a HashSet to eliminate duplicates within each subresult
        })
        .collect(); // Collect into a HashSet to eliminate duplicates across all results

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
                if board.can_place_piece(piece, (x as i32, y as i32)).is_ok() {
                    let mut new_board = board.clone(); // Clone the current board
                    new_board.place_piece(piece, (x as i32, y as i32)); // Place the piece
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
