use crate::date::Date;

use super::Curve;
use super::DateRatePair;
use super::TermRatePair;

pub struct DiscountCurve {
    date_rate_pairs: Vec<DateRatePair>,
}

impl DiscountCurve {
    pub fn from_term_rate_pair(term_rate_pair: Vec<TermRatePair>, start_date: Date) -> Self {
        let date_rate_pairs = vec![DateRatePair {
            date: Date::new(3, 1, 1995).unwrap(),
            rate: 0.42,
        }];
        DiscountCurve { date_rate_pairs }
    }
}

impl Curve for DiscountCurve {
    fn interpolate(&self) -> f64 {
        0.42
    }
}
