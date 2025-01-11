use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

use chrono::Datelike;
use std::str::FromStr;

#[derive(Debug)]
pub enum DayError {
    InvalidDay,
}

/// A struct representing a specific day.
pub struct Day {
    pub month: Month,
    pub day: MonthDay,
    pub weekday: Weekday,
}

impl Day {
    /// Creates a new `Day`.
    pub fn new(month: Month, day: MonthDay, weekday: Weekday) -> Result<Self, DayError> {
        let valid = match month {
            Month::April | Month::June | Month::September | Month::November => day <= MonthDay(30),
            Month::February => day <= MonthDay(29),
            _ => day <= MonthDay(31),
        };

        match valid {
            true => Ok(Day {
                month,
                day,
                weekday,
            }),
            false => Err(DayError::InvalidDay),
        }
    }

    pub fn today() -> Self {
        let now = chrono::Local::now();
        match Day::new(
            Month::from_str(&now.format("%B").to_string()).unwrap(),
            MonthDay::new(now.day() as u8).unwrap(),
            Weekday::from_str(&now.format("%A").to_string()).unwrap(),
        ) {
            Ok(day) => day,
            Err(_) => panic!("Invalid day."),
        }
    }
}

/// Represents the months of the year.
#[derive(EnumIter, EnumString, Display, AsRefStr)]
pub enum Month {
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
    pub fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Month::January => (0, 0),
            Month::February => (1, 0),
            Month::March => (2, 0),
            Month::April => (3, 0),
            Month::May => (0, 1),
            Month::June => (0, 2),
            Month::July => (0, 3),
            Month::August => (0, 4),
            Month::September => (0, 5),
            Month::October => (1, 5),
            Month::November => (2, 5),
            Month::December => (3, 5),
        }
    }

    pub fn day_count(&self) -> u8 {
        match self {
            Month::January
            | Month::March
            | Month::May
            | Month::July
            | Month::August
            | Month::October
            | Month::December => 31,
            Month::April | Month::June | Month::September | Month::November => 30,
            Month::February => 29,
        }
    }
}

/// Represents the days of the week.
#[derive(EnumIter, EnumString, Display, AsRefStr)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Weekday::Monday => (7, 0),
            Weekday::Tuesday => (8, 0),
            Weekday::Wednesday => (7, 1),
            Weekday::Thursday => (7, 2),
            Weekday::Friday => (7, 3),
            Weekday::Saturday => (8, 3),
            Weekday::Sunday => (8, 4),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct MonthDay(u8);

impl MonthDay {
    pub fn new(day: u8) -> Option<Self> {
        if day > 0 && day <= 31 {
            Some(MonthDay(day))
        } else {
            None
        }
    }

    pub fn to_coordinates(&self) -> (i32, i32) {
        match self.0 {
            1 => (4, 0),
            2 => (5, 0),
            3 => (6, 0),
            4 => (1, 1),
            5 => (2, 1),
            6 => (3, 1),
            7 => (4, 1),
            8 => (5, 1),
            9 => (6, 1),
            10 => (1, 2),
            11 => (2, 2),
            12 => (3, 2),
            13 => (4, 2),
            14 => (7, 5), // Weird one
            15 => (6, 2),
            16 => (1, 3),
            17 => (2, 3),
            18 => (3, 3),
            19 => (4, 3),
            20 => (5, 3),
            21 => (6, 3),
            22 => (1, 4),
            23 => (2, 4),
            24 => (3, 4),
            25 => (4, 4),
            26 => (5, 4),
            27 => (6, 4),
            28 => (4, 5),
            29 => (5, 5),
            30 => (6, 5),
            31 => (5, 2), // Weird one
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for MonthDay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
