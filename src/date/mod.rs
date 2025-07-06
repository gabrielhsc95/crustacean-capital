mod tests;
pub mod utils;
use crate::error::{Error, Result};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    pub fn new(day: u8, month: u8, year: u16) -> Result<Self> {
        if month > 12 {
            return Err(Error::DateInvalidMonth);
        }
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                if day > 31 {
                    return Err(Error::DateInvalidDay);
                }
            }
            4 | 6 | 9 | 11 => {
                if day > 30 {
                    return Err(Error::DateInvalidDay);
                }
            }
            2 => {
                if utils::is_leap_year(year) {
                    if day > 29 {
                        return Err(Error::DateInvalidDay);
                    }
                } else {
                    if day > 28 {
                        return Err(Error::DateInvalidDay);
                    }
                }
            }
            _ => return Err(Error::DateInvalidMonth),
        }
        Ok(Date { day, month, year })
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn year(&self) -> u16 {
        self.year
    }

    // https://en.wikipedia.org/wiki/Ordinal_date
    // same as python
    pub fn to_ordinal(&self) -> u128 {
        let adjusted_month: u128;
        let adjusted_year: u128;
        if self.month <= 2 {
            adjusted_month = self.month as u128 + 12;
            adjusted_year = self.year as u128 - 1;
        } else {
            adjusted_month = self.month as u128;
            adjusted_year = self.year as u128;
        }
        let year_contribution = 365 * adjusted_year + adjusted_year / 4 - adjusted_year / 100
            + adjusted_year / 400
            - 306;
        let month_contribution = (153 * (adjusted_month - 3) + 2) / 5;
        year_contribution + month_contribution + self.day as u128
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.to_ordinal() == other.to_ordinal()
    }

    fn ne(&self, other: &Self) -> bool {
        self.to_ordinal() != other.to_ordinal()
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_ordinal = self.to_ordinal();
        let other_ordinal = other.to_ordinal();
        if self_ordinal < other_ordinal {
            Some(Ordering::Less)
        } else if self_ordinal > other_ordinal {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for Date {}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_ordinal = self.to_ordinal();
        let other_ordinal = other.to_ordinal();
        if self_ordinal < other_ordinal {
            Ordering::Less
        } else if self_ordinal > other_ordinal {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
