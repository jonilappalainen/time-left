use chrono::{DateTime, Utc};

type TimeLeftTime = DateTime<Utc>;

pub struct TimeLeft {
    now: TimeLeftTime,
}

impl Default for TimeLeft {
    fn default() -> Self {
        return Self {now: Utc::now()};
    }
}

impl TimeLeft {
    pub fn new(now: TimeLeftTime) -> Self {
        return Self {now};
    }

    pub fn get_day_left(&self) -> f64 {
        return 0.0;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, NaiveDateTime, DateTime};

    use crate::{TimeLeft, TimeLeftTime};

    fn create_time_from(datetime: &str) -> TimeLeftTime {
        let t: NaiveDateTime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").unwrap();
        return DateTime::from_utc(t, Utc);
    }

    #[test]
    fn given_last_second_of_day_when_get_day_left_then_returns_zero() {
        let expected = 0.0;

        let actual = TimeLeft::new(create_time_from("2015-09-05 23:59:59")).get_day_left();

        assert_eq!(expected, actual);
    }

    #[test]
    fn given_first_second_of_day_when_get_day_left_then_returns_one() {
        let expected = 1.0;

        let actual = TimeLeft::new(create_time_from("2023-01-01 00:00:00")).get_day_left();

        assert_eq!(expected, actual);
    }

    #[test]
    fn given_middle_of_day_when_get_day_left_then_returns_half() {
        let expected = 0.5;

        let actual = TimeLeft::new(create_time_from("2023-01-01 12:00:00")).get_day_left();

        assert_eq!(expected, actual);
    }
}

fn main() {
    let tl = TimeLeft::default();
    println!("Hello, world! {0}", tl.get_day_left());
}
