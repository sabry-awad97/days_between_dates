use std::cmp::Ordering;

#[derive(Debug)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

impl Date {
    fn new(year: i32, month: i32, day: i32) -> Self {
        Self { year, month, day }
    }

    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
    }

    fn days_in_month(&self) -> i32 {
        match self.month {
            2 if self.is_leap_year() => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    fn next_day(&self) -> Self {
        if self.day < self.days_in_month() {
            Self::new(self.year, self.month, self.day + 1)
        } else if self.month < 12 {
            Self::new(self.year, self.month + 1, 1)
        } else {
            Self::new(self.year + 1, 1, 1)
        }
    }

    fn is_before(&self, other: &Self) -> bool {
        match self.year.cmp(&other.year) {
            Ordering::Greater => false,
            Ordering::Less => true,
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Greater => false,
                Ordering::Less => true,
                Ordering::Equal => self.day < other.day,
            },
        }
    }
}

fn days_between_dates(start_date: Date, end_date: Date) -> i32 {
    let mut days = 0;
    let mut current_date = start_date;

    while current_date.is_before(&end_date) {
        current_date = current_date.next_day();
        days += 1;
    }

    days
}

fn main() {
    let start_date = Date::new(2023, 10, 6);
    let end_date = Date::new(2023, 10, 10);

    let days = days_between_dates(start_date, end_date);
    println!("Number of days between the dates: {}", days);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_between_dates() {
        assert_eq!(
            days_between_dates(Date::new(2013, 1, 1), Date::new(2014, 1, 1)),
            365
        );
        assert_eq!(
            days_between_dates(Date::new(2012, 1, 1), Date::new(2013, 1, 1)),
            366
        );
        assert_eq!(
            days_between_dates(Date::new(2012, 9, 1), Date::new(2012, 9, 4)),
            3
        );
        assert_eq!(
            days_between_dates(Date::new(2011, 6, 30), Date::new(2012, 6, 30)),
            366
        );
        assert_eq!(
            days_between_dates(Date::new(2011, 1, 1), Date::new(2012, 8, 8)),
            585
        );
    }

    #[test]
    fn test_is_leap_year() {
        assert!(Date::new(2012, 1, 1).is_leap_year());
        assert!(Date::new(2000, 1, 1).is_leap_year());
        assert!(!Date::new(1900, 1, 1).is_leap_year());
        assert!(!Date::new(2013, 1, 1).is_leap_year());
    }
}
