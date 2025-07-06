use crate::date;

use super::utils::*;
use super::*;

// mod.rs
#[test]
fn errors_in_date_creation() {
    let invalid_day_31 = Date::new(31, 4, 2024);
    assert_eq!(invalid_day_31, Err(Error::DateInvalidDay));

    let invalid_day_32 = Date::new(32, 1, 2025);
    assert_eq!(invalid_day_32, Err(Error::DateInvalidDay));

    let invalid_day_feb_not_leap = Date::new(29, 2, 2023);
    assert_eq!(invalid_day_feb_not_leap, Err(Error::DateInvalidDay));

    let invalid_day_feb_leap = Date::new(30, 2, 2023);
    assert_eq!(invalid_day_feb_leap, Err(Error::DateInvalidDay));

    let invalid_month = Date::new(10, 13, 2025);
    assert_eq!(invalid_month, Err(Error::DateInvalidMonth));
}

#[test]
fn ordinal_dates() {
    let date_1 = Date::new(3, 1, 1995).unwrap();
    assert_eq!(date_1.to_ordinal(), 728296);
    let date_2 = Date::new(4, 5, 1989).unwrap();
    assert_eq!(date_2.to_ordinal(), 726226);
}

#[test]
fn comparison() {
    let date_1 = Date::new(3, 1, 1995).unwrap();
    let date_2 = Date::new(4, 5, 1989).unwrap();

    assert!(date_1 > date_2);
    assert!(date_2 < date_1);
    assert!(date_1 == date_1)
}

// utils.rs
#[test]
fn leap_year() {
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(1800));
    assert!(is_leap_year(2000));
}

#[test]
fn test_days_in_month() {
    assert_eq!(days_in_month(1, 2024), 31);
    assert_eq!(days_in_month(2, 2024), 29);
    assert_eq!(days_in_month(2, 2023), 28);
    assert_eq!(days_in_month(3, 2024), 31);
    assert_eq!(days_in_month(4, 2024), 30);
    assert_eq!(days_in_month(5, 2024), 31);
    assert_eq!(days_in_month(6, 2024), 30);
    assert_eq!(days_in_month(7, 2024), 31);
    assert_eq!(days_in_month(8, 2024), 31);
    assert_eq!(days_in_month(9, 2024), 30);
    assert_eq!(days_in_month(10, 2024), 31);
    assert_eq!(days_in_month(11, 2024), 30);
    assert_eq!(days_in_month(12, 2024), 31);
}
