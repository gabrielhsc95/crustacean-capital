use crate::calendar::Calendar;
use crate::date::Date;
use crate::frequency::Frequency;

pub struct DateGenerator {
    initial_date: Date,
    final_date: Date,
    // calendar: Calendar,
    frequency: Frequency,
    is_backwards: bool,
    current_date: Option<Date>,
}

impl DateGenerator {
    pub fn new(
        initial_date: Date,
        final_date: Date,
        // calendar: Calendar,
        frequency: Frequency,
        is_backwards: bool,
    ) -> Self {
        DateGenerator {
            initial_date,
            final_date,
            // calendar,
            frequency,
            is_backwards,
            current_date: None,
        }
    }
}

impl Iterator for DateGenerator {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_date.is_none() {
            self.current_date = Some(self.initial_date);
            return self.current_date.clone();
        } else if self.current_date >= Some(self.final_date) {
            None
        } else {
            let next_date = self.current_date.unwrap() + self.frequency.term();
            self.current_date = Some(next_date);
            return Some(next_date);
        }
    }
}
