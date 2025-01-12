use dialoguer::{theme::ColorfulTheme, Select};
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::calendar::{Day, Month, MonthDay, Weekday};

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
