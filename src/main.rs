mod board;
mod calendar;
mod piece;
mod pieces;

use board::Board;
use calendar::{Day, Month, MonthDay, Weekday};
use piece::{Piece, Rotation};
use pieces::{get_corner_piece, get_default_pieces};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use strum::IntoEnumIterator;

/// Configuration
const BOARD_WIDTH: usize = 9;
const BOARD_HEIGHT: usize = 6;
const MISSING_CORNER_COORDINATES: (i32, i32) = (8, 5);

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

    let day = if args.today {
        Day::today()
    } else {
        select_day()
    };

    // Create the three calendar pieces.
    let month_piece = Piece::new('☻', vec![(0, 0)], (255, 255, 255), (0, 0, 0));
    let day_piece = Piece::new('◉', vec![(0, 0)], (255, 255, 255), (0, 0, 0));
    let weekday_piece = Piece::new('☼', vec![(0, 0)], (255, 255, 255), (0, 0, 0));

    // Create the corner piece.
    let corner_piece = get_corner_piece();

    // Define the initial board.
    let mut initial_board = Board::new(BOARD_WIDTH, BOARD_HEIGHT, '·');

    // Place the calendar pieces on the board.
    initial_board.place_piece(&month_piece, Rotation::Zero, day.month.to_coordinates());
    initial_board.place_piece(&day_piece, Rotation::Zero, day.day.to_coordinates());
    initial_board.place_piece(&weekday_piece, Rotation::Zero, day.weekday.to_coordinates());

    // Corner piece
    initial_board.place_piece(&corner_piece, Rotation::Zero, MISSING_CORNER_COORDINATES);

    println!("{}, {} {}", day.weekday, day.month, day.day);
    initial_board.display();
    println!();

    // Define the pieces to place
    let owned_pieces = get_default_pieces();
    let mut pieces: Vec<&Piece> = owned_pieces.iter().collect();

    if args.show_pieces {
        println!("Pieces to place:");
        for piece in &pieces {
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

    // Generate all valid boards that place all pieces
    let final_boards = initial_board.find_boards_placing_all_pieces(
        &mut pieces,
        &AtomicBool::new(false),
        args.all,
    );

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

    // Choose a day of the week
    let weekdays = Weekday::iter().map(|w| w.to_string()).collect::<Vec<_>>();
    let weekday_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a day of the week")
        .items(&weekdays)
        .default(0)
        .interact()
        .unwrap();
    let weekday = Weekday::from_str(&weekdays[weekday_index]).unwrap();

    match Day::new(month, day, weekday) {
        Ok(day) => day,
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
