use dialoguer::{theme::ColorfulTheme, Select};
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::{
    board::Board,
    calendar::{Day, Month, MonthDay, Weekday},
    piece::{Piece, Placement, Rotation},
};
use clap::Parser;

/// Command-line arguments
#[derive(Parser)]
pub struct Args {
    /// Show all solutions, not just the first one.
    #[arg(long)]
    pub all: bool,

    ///  Show the pieces to place.
    #[arg(long = "show-pieces")]
    pub show_pieces: bool,

    /// Use today's date.
    #[arg(long)]
    pub today: bool,

    /// Use raw text output
    #[arg(long)]
    pub raw: bool,

    /// The month
    #[arg(long)]
    pub month: Option<Month>,

    /// The day of the month
    #[arg(long)]
    pub day: Option<MonthDay>,

    /// The day of the week
    #[arg(long)]
    pub weekday: Option<Weekday>,

    /// Hint <HINT> pieces instead of giving full solutions
    #[arg(long)]
    pub hint: Option<usize>,
}

impl Args {
    pub fn get_day(&self) -> Day {
        if self.today {
            Day::today()
        } else if let (Some(month), Some(day), Some(weekday)) = (
            self.month.as_ref(),
            self.day.as_ref(),
            self.weekday.as_ref(),
        ) {
            Day::new(month.clone(), day.clone(), weekday.clone()).unwrap()
        } else {
            select_day()
        }
    }
}

/// Gets input from the user to create a `Day` struct
pub fn select_day() -> Day {
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

/// Shows pieces for placement
pub fn show_pieces(pieces: &[Piece]) {
    println!("Pieces to place:");
    for piece in pieces {
        // Make an example board just big enough to display this piece.
        let mut example_board = Board::new(
            piece.get_dimensions_of_shape(Rotation::Zero, false).0 as usize,
            piece.get_dimensions_of_shape(Rotation::Zero, false).1 as usize,
            ' ',
        );
        // Place the piece in the top-left corner for display
        example_board.place_piece(piece, Placement::new(Rotation::Zero, false, (0, 0)));
        example_board.display();
        println!();
    }
}
