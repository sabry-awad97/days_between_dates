use std::cmp::Ordering;

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn next_day(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    if day < days_in_month(year, month) {
        (year, month, day + 1)
    } else if month < 12 {
        (year, month + 1, 1)
    } else {
        (year + 1, 1, 1)
    }
}

fn date_is_before(year1: i32, month1: i32, day1: i32, year2: i32, month2: i32, day2: i32) -> bool {
    match year1.cmp(&year2) {
        Ordering::Greater => false,
        Ordering::Less => true,
        Ordering::Equal => match month1.cmp(&month2) {
            Ordering::Greater => false,
            Ordering::Less => true,
            Ordering::Equal => day1 < day2,
        },
    }
}

fn days_between_dates(
    year1: i32,
    month1: i32,
    day1: i32,
    year2: i32,
    month2: i32,
    day2: i32,
) -> i32 {
    let mut days = 0;
    let mut current_date = (year1, month1, day1);

    while date_is_before(
        current_date.0,
        current_date.1,
        current_date.2,
        year2,
        month2,
        day2,
    ) {
        current_date = next_day(current_date.0, current_date.1, current_date.2);
        days += 1;
    }

    days
}

fn main() {
    let days = days_between_dates(2023, 10, 6, 2023, 10, 10);
    println!("Number of days between the dates: {}", days);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_between_dates() {
        assert_eq!(days_between_dates(2013, 1, 1, 2014, 1, 1), 365);
        assert_eq!(days_between_dates(2012, 1, 1, 2013, 1, 1), 366);
        assert_eq!(days_between_dates(2012, 9, 1, 2012, 9, 4), 3);
        assert_eq!(days_between_dates(2011, 6, 30, 2012, 6, 30), 366);
        assert_eq!(days_between_dates(2011, 1, 1, 2012, 8, 8), 585);
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2012));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2013));
    }
}
