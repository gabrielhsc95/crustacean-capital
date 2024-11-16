mod calendar;
mod coupon;
mod coupon_date_generator;
mod curve;
mod date;
mod error;
mod term;

use crate::calendar::Calendar;

fn main() {
    let cal = Calendar::default();
    println!("{cal:?}");
}
