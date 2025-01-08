use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

/// A struct representing a specific day.
pub struct Day {
    month: Month,
    day: u8,
    weekday: Weekday,
}

impl Day {
    /// Creates a new `Day`.
    pub fn new(month: Month, day: u8, weekday: Weekday) -> Self {
        Day {
            month,
            day,
            weekday,
        }
    }

    /// Displays the details of the `Day`.
    pub fn display(&self) {
        println!(
            "Selected Date:\nMonth: {}\nDay: {}\nDay of Week: {}",
            self.month.to_string(),
            self.day,
            self.weekday.to_string()
        );
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
