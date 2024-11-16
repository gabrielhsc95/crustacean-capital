use super::*;

fn create_test_calendar() -> Calendar {
    Calendar::new(
        String::from("Test"),
        Vec::new(),
        vec![
            Date::new(25, 12, 2024).unwrap(),
            Date::new(1, 1, 2025).unwrap(),
        ],
        DayCountConvention::DDCActual365,
    )
}

#[test]
//test sorting at creation
fn new_calendar() {
    let cal = Calendar::new(
        String::from("Test"),
        Vec::new(),
        vec![
            Date::new(1, 1, 2025).unwrap(),
            Date::new(25, 12, 2024).unwrap(),
        ],
        DayCountConvention::DDCActual365,
    );
    assert_eq!(
        cal.holidays,
        vec![
            Date::new(25, 12, 2024).unwrap(),
            Date::new(1, 1, 2025).unwrap()
        ]
    );
}

#[test]
fn add_holiday_to_calendar() {
    let mut cal = create_test_calendar();
    cal.add_holiday(Date::new(28, 12, 2024).unwrap());
    assert_eq!(
        cal.holidays,
        vec![
            Date::new(25, 12, 2024).unwrap(),
            Date::new(28, 12, 2024).unwrap(),
            Date::new(1, 1, 2025).unwrap(),
        ]
    )
}

#[test]
fn distance() {
    let cal = create_test_calendar();
    let date_1 = Date::new(3, 1, 1995).unwrap();
    let date_2 = Date::new(4, 5, 1989).unwrap();

    assert_eq!(cal.distance(date_2, date_1), 5.671232876712328)
}
