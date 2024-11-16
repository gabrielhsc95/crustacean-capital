use crate::date::Date;

pub enum CouponKind {
    Fixed,
}

struct Coupon {
    kind: CouponKind,
    date: Date,
    rate: f64,
    distance: f64,
    discount_rate: f64,
    discount: f64,
    face_value: f64,
    discounted_value: f64,
}
