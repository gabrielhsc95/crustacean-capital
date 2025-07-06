use crate::date::utils::days_in_month;
use crate::date::Date;
use std::ops::Add;
mod test;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TermUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Term {
    value: u16,
    unit: TermUnit,
}

impl Term {
    pub fn new(value: u16, unit: TermUnit) -> Self {
        Term { value, unit }
    }
}

impl Add<Term> for Date {
    type Output = Date;

    fn add(self, rhs: Term) -> Self::Output {
        match rhs.unit {
            TermUnit::Day => add_days(&self, rhs.value as u32),
            TermUnit::Week => {
                let days = rhs.value * 7;
                add_days(&self, days as u32)
            }
            TermUnit::Month => add_months(&self, rhs.value as u32),
            TermUnit::Year => add_years(&self, rhs.value as u16),
        }
    }
}

impl Add<Term> for Term {
    type Output = Term;

    fn add(self, rhs: Term) -> Self::Output {
        match self.unit {
            TermUnit::Day => match rhs.unit {
                TermUnit::Day => Term {
                    value: self.value + rhs.value,
                    unit: TermUnit::Day,
                },
                TermUnit::Week => Term {
                    value: self.value + rhs.value * 7,
                    unit: TermUnit::Day,
                },
                TermUnit::Month => Term {
                    value: self.value + rhs.value * 30,
                    unit: TermUnit::Day,
                },
                TermUnit::Year => Term {
                    value: self.value + rhs.value * 365,
                    unit: TermUnit::Day,
                },
            },
            TermUnit::Week => match rhs.unit {
                TermUnit::Day => Term {
                    value: self.value * 7 + rhs.value,
                    unit: TermUnit::Day,
                },
                TermUnit::Week => Term {
                    value: self.value + rhs.value,
                    unit: TermUnit::Week,
                },
                TermUnit::Month => Term {
                    value: self.value + rhs.value * 4,
                    unit: TermUnit::Week,
                },
                TermUnit::Year => Term {
                    value: self.value + rhs.value * 52,
                    unit: TermUnit::Week,
                },
            },
            TermUnit::Month => match rhs.unit {
                TermUnit::Day => Term {
                    value: self.value + rhs.value * 30,
                    unit: TermUnit::Day,
                },
                TermUnit::Week => Term {
                    value: self.value * 4 + rhs.value,
                    unit: TermUnit::Week,
                },
                TermUnit::Month => Term {
                    value: self.value + rhs.value,
                    unit: TermUnit::Month,
                },
                TermUnit::Year => Term {
                    value: self.value + rhs.value * 12,
                    unit: TermUnit::Month,
                },
            },
            TermUnit::Year => match rhs.unit {
                TermUnit::Day => Term {
                    value: self.value + rhs.value * 365,
                    unit: TermUnit::Day,
                },
                TermUnit::Week => Term {
                    value: self.value * 52 + rhs.value,
                    unit: TermUnit::Week,
                },
                TermUnit::Month => Term {
                    value: self.value + rhs.value * 12,
                    unit: TermUnit::Month,
                },
                TermUnit::Year => Term {
                    value: self.value + rhs.value,
                    unit: TermUnit::Year,
                },
            },
        }
    }
}

pub fn add_days(date: &Date, days: u32) -> Date {
    let mut day = date.day() as u32;
    let mut month = date.month();
    let mut year = date.year();

    let mut remaining_days = days;
    while remaining_days > 0 {
        let days_in_month = days_in_month(month, year) as u32;
        if day as u32 + remaining_days <= days_in_month {
            day += remaining_days;
            remaining_days = 0;
        } else {
            remaining_days -= days_in_month - day as u32;
            day = 0; // it will be set properly in the next iteration
            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }
        }
    }
    Date::new(day as u8, month, year).expect("There is business logic bug in add_days")
}

fn cap_day(day: u8, month: u8, year: u16) -> u8 {
    let days_in_month = days_in_month(month, year);
    day.min(days_in_month)
}

pub fn add_months(date: &Date, months: u32) -> Date {
    let mut day = date.day();
    let mut month = date.month();
    let mut year = date.year();

    let new_month = month as u32 + months;
    month = (new_month % 12) as u8;
    year += new_month as u16 / 12;
    day = cap_day(day, month, year);
    Date::new(day, month, year).expect("There is a business logic bug in add_months")
}

pub fn add_years(date: &Date, years: u16) -> Date {
    let mut day = date.day();
    let month = date.month();
    let mut year = date.year();
    year += years;
    day = cap_day(day, month, year);
    Date::new(day, date.month(), year).expect("There is a business bug logic in add_years")
}
