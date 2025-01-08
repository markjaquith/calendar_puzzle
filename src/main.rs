mod board;
mod calendar;
mod piece;
mod pieces;

use board::Board;
use calendar::{Day, Month, Weekday};
use piece::{Piece, Rotation};
use pieces::{get_corner_piece, get_default_pieces};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

/// Command-line arguments
#[derive(Parser)]
struct Args {
    // Stop as soon as the first valid board is found
    #[arg(short, long)]
    first: bool,

    //  Show the pieces to place.
    #[arg(long = "show-pieces")]
    show_pieces: bool,
}

fn main() {
    let args = Args::parse();
    let first = args.first;
    let show_pieces = args.show_pieces;

    // Atomic flag for tracking whether a valid board has been found.
    let found = AtomicBool::new(false);

    // Define the initial board
    let width = 9;
    let height = 6;
    let mut initial_board = Board::new(width, height, '·');
    initial_board.place_piece(
        &Piece::new('☻', vec![(0, 0)], (255, 255, 255), (0, 0, 0)),
        8,
        0,
    );
    initial_board.place_piece(
        &Piece::new('☺', vec![(0, 0)], (255, 255, 255), (0, 0, 0)),
        0,
        0,
    );
    initial_board.place_piece(
        &Piece::new('☼', vec![(0, 0)], (255, 255, 255), (0, 0, 0)),
        4,
        1,
    );

    // Corner piece
    initial_board.place_piece(&get_corner_piece(), 8, 5);

    println!("Solving board:");
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
            example_board.place_piece(piece, 0, 0);
            example_board.display();
            println!(); // Blank line between boards
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
