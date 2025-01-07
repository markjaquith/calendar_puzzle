/// A struct representing a specific day.
#[derive(Debug)]
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
            self.month.as_str(),
            self.day,
            self.weekday.as_str()
        );
    }
}

/// Represents the months of the year.
#[derive(Debug, Clone, Copy)]
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
    /// Converts an index (0-11) to a `Month`.
    pub fn from_index(index: usize) -> Self {
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
    pub fn as_str(&self) -> &'static str {
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
    /// Converts an index (0-6) to a `Weekday`.
    pub fn from_index(index: usize) -> Self {
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
    pub fn as_str(&self) -> &'static str {
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
