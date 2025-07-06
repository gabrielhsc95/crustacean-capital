use crate::calendar::Calendar;
use crate::curve;
use crate::date::Date;
use crate::term;

fn default_curve_test() -> curve::DiscountCurve {
    curve::DiscountCurve {
        start_date: Date::new(1, 1, 2025).unwrap(),
        calendar: Calendar::default(),
        distance_rate_pairs: vec![
            curve::DistanceRatePair {
                distance: 2.0 / 365.0,
                rate: 0.01,
            },
            curve::DistanceRatePair {
                distance: 4.0 / 365.0,
                rate: 0.02,
            },
        ],
    }
}

#[test]
fn test_from_term_rate_pairs() {
    let term_rate_pairs = vec![
        curve::TermRatePair {
            term: term::Term::new(4, term::TermUnit::Day),
            rate: 0.02,
        },
        curve::TermRatePair {
            term: term::Term::new(2, term::TermUnit::Day),
            rate: 0.01,
        },
    ];
    let start_date = Date::new(1, 1, 2025).unwrap();
    let calendar = Calendar::default();
    let curve = curve::DiscountCurve::from_term_rate_pairs(&term_rate_pairs, start_date, calendar);
    assert_eq!(curve, default_curve_test());
}

fn test_from_date_rate_pairs() {
    let date_rate_pairs = vec![
        curve::DateRatePair {
            date: Date::new(5, 1, 2025).unwrap(),
            rate: 0.02,
        },
        curve::DateRatePair {
            date: Date::new(2, 1, 2025).unwrap(),
            rate: 0.01,
        },
    ];
    let start_date = Date::new(1, 1, 2025).unwrap();
    let calendar = Calendar::default();
    let curve = curve::DiscountCurve::from_date_rate_pairs(&date_rate_pairs, start_date, calendar);
    assert_eq!(curve, default_curve_test());
}

fn test_new() {
    let distance_rate_pairs = vec![
        curve::DistanceRatePair {
            distance: 4.0 / 365.0,
            rate: 0.02,
        },
        curve::DistanceRatePair {
            distance: 2.0 / 365.0,
            rate: 0.01,
        },
    ];
    let start_date = Date::new(1, 1, 2025).unwrap();
    let calendar = Calendar::default();
    let curve = curve::DiscountCurve::new(distance_rate_pairs, start_date, calendar);
    assert_eq!(curve, default_curve_test());
}

fn test_piecewise_linear_interpolation() {
    let curve = default_curve_test();
    let date_less = Date::new(1, 1, 2025).unwrap();
    assert_eq!(curve.piecewise_linear_interpolation(&date_less), 0.01);
    let date_greater = Date::new(5, 1, 2025).unwrap();
    assert_eq!(curve.piecewise_linear_interpolation(&date_greater), 0.02);
    let date_between = Date::new(3, 1, 2025).unwrap();
    assert_eq!(curve.piecewise_linear_interpolation(&date_between), 0.015);
    let date_same = Date::new(2, 1, 2025).unwrap();
    assert_eq!(curve.piecewise_linear_interpolation(&date_same), 0.01);
}
