use crate::date::Date;
use crate::term::Term;

mod discount_curve;

struct DateRatePair {
    date: Date,
    rate: f64,
}

struct TermRatePair {
    term: Term,
    rate: f64,
}

trait Curve {
    fn interpolate(&self) -> f64;
}
