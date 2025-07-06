use crate::date::Date;
use crate::term;

#[test]
#[should_panic(expected = "Invalid month")]
fn test_days_in_month_panic() {
    term::days_in_month(13, 2024);
}

#[test]
fn test_add_days() {
    let date_1 = Date::new(3, 1, 1995).unwrap();
    let new_date_1 = term::add_days(&date_1, 10);
    assert_eq!(new_date_1, Date::new(13, 1, 1995).unwrap());
    let new_date_2 = term::add_days(&date_1, 30);
    assert_eq!(new_date_2, Date::new(2, 2, 1995).unwrap());
    let new_date_3 = term::add_days(&date_1, 100);
    assert_eq!(new_date_3, Date::new(13, 4, 1995).unwrap());
    let new_date_4 = term::add_days(&date_1, 365);
    assert_eq!(new_date_4, Date::new(3, 1, 1996).unwrap());
    let date_2 = Date::new(28, 2, 2024).unwrap();
    let new_date_5 = term::add_days(&date_2, 1);
    assert_eq!(new_date_5, Date::new(29, 2, 2024).unwrap());
}

#[test]
fn test_add_months() {
    let date_1 = Date::new(3, 1, 1995).unwrap();
    let new_date_1 = term::add_months(&date_1, 1);
    assert_eq!(new_date_1, Date::new(3, 2, 1995).unwrap());
    let new_date_2 = term::add_months(&date_1, 6);
    assert_eq!(new_date_2, Date::new(3, 7, 1995).unwrap());
    let new_date_2 = term::add_months(&date_1, 12);
    assert_eq!(new_date_2, Date::new(3, 1, 1996).unwrap());
    let new_date_3 = term::add_months(&date_1, 100);
    assert_eq!(new_date_3, Date::new(3, 5, 2003).unwrap());
    let date_2 = Date::new(31, 1, 2024).unwrap();
    let new_date_4 = term::add_months(&date_2, 1);
    assert_eq!(new_date_4, Date::new(29, 2, 2024).unwrap());
    let new_date_5 = term::add_months(&date_2, 2);
    assert_eq!(new_date_5, Date::new(31, 3, 2024).unwrap());
    let new_date_6 = term::add_months(&date_2, 3);
    assert_eq!(new_date_6, Date::new(30, 4, 2024).unwrap());
    let date_3 = Date::new(31, 1, 2023).unwrap();
    let new_date_7 = term::add_months(&date_3, 1);
    assert_eq!(new_date_7, Date::new(28, 2, 2023).unwrap());
}

#[test]
fn test_add_years() {
    let date_1 = Date::new(3, 1, 1995).unwrap();
    let new_date_1 = term::add_years(&date_1, 1);
    assert_eq!(new_date_1, Date::new(3, 1, 1996).unwrap());
    let new_date_2 = term::add_years(&date_1, 10);
    assert_eq!(new_date_2, Date::new(3, 1, 2005).unwrap());
    let date_2 = Date::new(29, 2, 2024).unwrap();
    let new_date_3 = term::add_years(&date_2, 1);
    assert_eq!(new_date_3, Date::new(28, 2, 2025).unwrap());
    let date_3 = Date::new(28, 2, 2023).unwrap();
    assert_eq!(term::add_years(&date_3, 1), Date::new(28, 2, 2024).unwrap())
}

#[test]
fn test_term_add() {
    let term_1d = term::Term {
        value: 1,
        unit: term::TermUnit::Day,
    };
    let term_1w = term::Term {
        value: 1,
        unit: term::TermUnit::Week,
    };
    let term_1m = term::Term {
        value: 1,
        unit: term::TermUnit::Month,
    };
    let term_1y = term::Term {
        value: 1,
        unit: term::TermUnit::Year,
    };

    assert_eq!(
        term_1d + term_1d,
        term::Term {
            value: 2,
            unit: term::TermUnit::Day,
        }
    );
    assert_eq!(
        term_1d + term_1w,
        term::Term {
            value: 8,
            unit: term::TermUnit::Day,
        }
    );
    assert_eq!(
        term_1d + term_1m,
        term::Term {
            value: 31,
            unit: term::TermUnit::Day,
        }
    );
    assert_eq!(
        term_1d + term_1y,
        term::Term {
            value: 366,
            unit: term::TermUnit::Day,
        }
    );
    assert_eq!(
        term_1w + term_1d,
        term::Term {
            value: 8,
            unit: term::TermUnit::Day,
        }
    );
    assert_eq!(
        term_1w + term_1w,
        term::Term {
            value: 2,
            unit: term::TermUnit::Week,
        }
    );
    assert_eq!(
        term_1w + term_1m,
        term::Term {
            value: 5,
            unit: term::TermUnit::Week,
        }
    );
    assert_eq!(
        term_1w + term_1y,
        term::Term {
            value: 53,
            unit: term::TermUnit::Week,
        }
    );
    assert_eq!(
        term_1m + term_1d,
        term::Term {
            value: 31,
            unit: term::TermUnit::Day,
        }
    );
    assert_eq!(
        term_1m + term_1w,
        term::Term {
            value: 5,
            unit: term::TermUnit::Week,
        }
    );
    assert_eq!(
        term_1m + term_1m,
        term::Term {
            value: 2,
            unit: term::TermUnit::Month,
        }
    );
    assert_eq!(
        term_1m + term_1y,
        term::Term {
            value: 13,
            unit: term::TermUnit::Month,
        }
    );
    assert_eq!(
        term_1y + term_1d,
        term::Term {
            value: 366,
            unit: term::TermUnit::Day,
        }
    );
    assert_eq!(
        term_1y + term_1w,
        term::Term {
            value: 53,
            unit: term::TermUnit::Week,
        }
    );
    assert_eq!(
        term_1y + term_1m,
        term::Term {
            value: 13,
            unit: term::TermUnit::Month,
        }
    );
    assert_eq!(
        term_1y + term_1y,
        term::Term {
            value: 2,
            unit: term::TermUnit::Year,
        }
    );
}
