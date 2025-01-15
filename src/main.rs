mod board;
mod calendar;
mod cli;
mod piece;
mod pieces;

use board::Board;
use clap::Parser;
use cli::{show_pieces, Args};
use piece::Piece;
use pieces::Pieces;

use std::sync::atomic::AtomicBool;

fn main() {
    let args = Args::parse();

    let day = args.get_day();
    let board = Board::make(&day);

    if !args.raw {
        println!("{}, {} {}", day.weekday, day.month, day.day);
        board.display();
        println!();
    }

    // Define the pieces to place
    let default_pieces = Pieces::get_defaults_for_board(&board);

    // Handle --show-pieces flag
    if !args.raw && args.show_pieces {
        show_pieces(&default_pieces);
    }

    // Create a mutable list of references to the default pieces.
    let mut pieces: Vec<&Piece> = default_pieces.iter().collect();

    // Generate all valid boards that place all pieces.
    let mut final_boards = board
        .find_boards_placing_all_pieces(
            &mut pieces,
            &AtomicBool::new(false), // Whether any solutions have been found
            args.all,                // Whether to find all solutions
        )
        .into_iter()
        .collect::<Vec<_>>();

    final_boards.sort_by(|a, b| a.serialize().cmp(&b.serialize()));

    for (i, board) in final_boards.iter().enumerate() {
        // Only display the solution number if --all is used
        if !args.raw && args.all {
            println!("Solution {}:", i + 1);
        }

        if args.raw {
            println!("{}", board.serialize());
        } else {
            board.display();
            println!();
        }
    }
}
