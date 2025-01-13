mod board;
mod calendar;
mod cli;
mod piece;
mod pieces;

use board::Board;
use calendar::Day;
use clap::Parser;
use cli::{select_day, Args};
use piece::{Piece, Rotation};
use pieces::Pieces;

use std::sync::atomic::AtomicBool;

fn main() {
    let args = Args::parse();

    let day = if args.today {
        Day::today()
    } else {
        select_day()
    };

    let board = Board::make(&day);
    println!("{}, {} {}", day.weekday, day.month, day.day);
    board.display();
    println!();

    // Define the pieces to place
    let default_pieces = Pieces::get_default();

    if args.show_pieces {
        println!("Pieces to place:");
        for piece in &default_pieces {
            // Make an example board just big enough to display this piece.
            let mut example_board = Board::new(
                piece.get_default_dimensions().0 as usize,
                piece.get_default_dimensions().1 as usize,
                ' ',
            );
            // Place the piece in the top-left corner for display
            example_board.place_piece(piece, Rotation::Zero, (0, 0));
            example_board.display();
            println!();
        }
    }

    // Create a mutable list of references to the default pieces.
    let mut pieces: Vec<&Piece> = default_pieces.iter().collect();

    // Generate all valid boards that place all pieces.
    let final_boards =
        board.find_boards_placing_all_pieces(&mut pieces, &AtomicBool::new(false), args.all);

    if !args.all && final_boards.len() > 1 {
        println!(
            "Found {} boards that successfully place all pieces:",
            final_boards.len()
        );
    }

    for (i, board) in final_boards.iter().enumerate() {
        if args.all {
            println!("Solution {}:", i + 1);
        }

        board.display();
        println!();
    }
}
