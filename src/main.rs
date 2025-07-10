mod calendar;
mod coupon;
mod curve;
mod date;
mod error;
mod frequency;
mod term;

use crate::calendar::Calendar;
use crate::date as d;
use crate::frequency as f;
use crate::term as t;

fn main() {
    let cal = Calendar::default();
    println!("{cal:?}");
    let generator = d::generator::DateGenerator::new(
        d::Date::new(1, 1, 2023).unwrap(),
        d::Date::new(1, 12, 2025).unwrap(),
        f::Frequency::new(t::Term::new(1, term::TermUnit::Month), 30.0),
        false,
    );
    for date in generator {
        println!("{date:?}");
        println!("----------------")
    }
}
