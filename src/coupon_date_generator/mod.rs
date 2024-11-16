use crate::calendar::Calendar;
use crate::coupon::CouponKind;
use crate::date::Date;

struct Coupon {
    date: Date,
    distance: f64,
    rate: f64,
}
struct CouponDateGenerator {
    kind: CouponKind,
    current: Date,
    start_date: Date,
    end_date: Date,
    calendar: Calendar,
    is_backwards: bool,
}

impl Iterator for CouponDateGenerator {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        match self.kind {
            CouponKind::Fixed => Some(Date::new(3, 1, 1995).unwrap()),
        }
    }
}
