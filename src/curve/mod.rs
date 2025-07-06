use crate::calendar::{self, Calendar};
use crate::date::Date;
use crate::term::Term;
use std::cmp::Ordering;
mod tests;

pub struct DateRatePair {
    date: Date,
    rate: f64,
}

pub struct TermRatePair {
    term: Term,
    rate: f64,
}

#[derive(Debug, PartialEq)]
pub struct DistanceRatePair {
    distance: f64,
    rate: f64,
}

impl Eq for DistanceRatePair {}

impl PartialOrd for DistanceRatePair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for DistanceRatePair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}
#[derive(Debug, PartialEq)]
pub struct DiscountCurve {
    start_date: Date,
    calendar: Calendar,
    // TODO: validate pair to avoid duplicates in distance
    distance_rate_pairs: Vec<DistanceRatePair>,
}

impl DiscountCurve {
    pub fn from_term_rate_pairs(
        term_rate_pairs: &Vec<TermRatePair>,
        start_date: Date,
        calendar: Calendar,
    ) -> Self {
        let mut distance_rate_pairs = Vec::new();
        for term_rate_pair in term_rate_pairs {
            let date = start_date + term_rate_pair.term;
            let distance = calendar.distance(&start_date, &date);
            let distance_rate_pair = DistanceRatePair {
                distance,
                rate: term_rate_pair.rate,
            };
            distance_rate_pairs.push(distance_rate_pair);
        }
        distance_rate_pairs.sort();
        DiscountCurve {
            start_date,
            calendar,
            distance_rate_pairs,
        }
    }

    pub fn from_date_rate_pairs(
        date_rate_pairs: &Vec<DateRatePair>,
        start_date: Date,
        calendar: Calendar,
    ) -> Self {
        // TODO: what if there is date that is before start_date,
        // my initial thought is that I don't think it is technically wrong, just weird
        let mut distance_rate_pairs = Vec::new();
        for date_rate_pair in date_rate_pairs {
            let distance = calendar.distance(&start_date, &date_rate_pair.date);
            let distance_rate_pair = DistanceRatePair {
                distance,
                rate: date_rate_pair.rate,
            };
            distance_rate_pairs.push(distance_rate_pair);
        }
        distance_rate_pairs.sort();
        DiscountCurve {
            start_date,
            calendar,
            distance_rate_pairs,
        }
    }

    pub fn new(
        mut distance_rate_pairs: Vec<DistanceRatePair>,
        start_date: Date,
        calendar: Calendar,
    ) -> Self {
        distance_rate_pairs.sort();
        DiscountCurve {
            start_date,
            calendar,
            distance_rate_pairs,
        }
    }

    fn piecewise_linear_interpolation(&self, date: &Date) -> f64 {
        let distance = self.calendar.distance(&self.start_date, date);
        if distance < self.distance_rate_pairs[0].rate {
            return self.distance_rate_pairs[0].rate;
        } else if distance > self.distance_rate_pairs[self.distance_rate_pairs.len() - 1].rate {
            return self.distance_rate_pairs[self.distance_rate_pairs.len() - 1].rate;
        }
        let mut smaller_index: usize = 0;
        for (i, distance_rate_pair) in self.distance_rate_pairs.iter().enumerate() {
            if distance == distance_rate_pair.distance {
                return distance_rate_pair.rate;
            } else if distance < distance_rate_pair.distance {
                smaller_index = i;
            }
        }
        // it should not panic because if the smaller index was the last one, then it should have
        // returned already with distance > self.distance_rate_pairs[self.distance_rate_pairs.len() - 1].rate
        let greater_index = smaller_index + 1;
        let greater_distance = self.distance_rate_pairs[greater_index].distance;
        let greater_rate = self.distance_rate_pairs[greater_index].rate;

        let smaller_distance = self.distance_rate_pairs[smaller_index].distance;
        let smaller_rate = self.distance_rate_pairs[smaller_index].rate;

        let slope = (greater_rate - smaller_rate) / (greater_distance - smaller_distance);
        let intercept = smaller_rate - slope * smaller_distance;
        slope * distance + intercept
    }
}
