mod tests;

use crate::date::Date;

#[derive(Debug, PartialEq)]
pub enum DaysOfTheWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
#[derive(Debug, PartialEq)]
// https://en.wikipedia.org/wiki/Day_count_convention
// some DayCountConvention names start with a number,
// so every one will start with DCC (Day Count Convention)
pub enum DayCountConvention {
    DDCActual365,
}
#[derive(Debug, PartialEq)]
pub struct Calendar {
    name: String,
    weekend: Vec<DaysOfTheWeek>,
    holidays: Vec<Date>,
    day_count_convention: DayCountConvention,
}

impl Calendar {
    pub fn new(
        name: String,
        weekend: Vec<DaysOfTheWeek>,
        holidays: Vec<Date>,
        day_count_convention: DayCountConvention,
    ) -> Self {
        let mut sorted_holidays = holidays.clone();
        sorted_holidays.sort();
        Calendar {
            name,
            weekend,
            holidays: sorted_holidays,
            day_count_convention,
        }
    }
}

impl Calendar {
    pub fn add_holiday(&mut self, date: Date) {
        let mut new_holidays = self.holidays.clone();
        new_holidays.push(date);
        new_holidays.sort();
        self.holidays = new_holidays;
    }

    pub fn distance(&self, start: &Date, end: &Date) -> f64 {
        match self.day_count_convention {
            DayCountConvention::DDCActual365 => {
                (end.to_ordinal() as f64 - start.to_ordinal() as f64) / 365.0
            }
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Calendar::new(
            String::from("Default Calendar"),
            vec![DaysOfTheWeek::Saturday, DaysOfTheWeek::Sunday],
            Vec::new(),
            DayCountConvention::DDCActual365,
        )
    }
}
