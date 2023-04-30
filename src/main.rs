
use chrono::{DateTime, Utc, Timelike, Days};

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
        const DAY_AS_SECONDS: i64 = 24*60*60;
        let mut end_of_day = self.now.clone();
        end_of_day = end_of_day.checked_add_days(Days::new(1)).unwrap();
        end_of_day = end_of_day.with_hour(0).unwrap();
        end_of_day = end_of_day.with_minute(0).unwrap();
        end_of_day = end_of_day.with_second(0).unwrap();
        let left_seconds = end_of_day.timestamp() - self.now.timestamp();
        let perc = left_seconds as f64 / DAY_AS_SECONDS as f64;
        return round(perc, 3);
    }
}

pub struct TimeWindow {
    t1: TimeLeftTime,
    t2: TimeLeftTime,
    tx: Option<TimeLeftTime>
}

impl TimeWindow {
    pub fn new(t1: TimeLeftTime, t2: TimeLeftTime) -> Self {
        return Self {
            t1,
            t2,
            tx: None
        }
    }

    pub fn set_point(&mut self, t: TimeLeftTime) {
        self.tx = Some(t);
    }

    pub fn get_percentage(&self) -> f64 {
        let t1 = self.t1.timestamp_millis();
        let t2 = self.t2.timestamp_millis();
        let tx = self.tx.unwrap().timestamp_millis();
        let total = t2 - t1;
        let past = tx - t1;
        // let left = t2 - tx;
        let perc = past as f64 / total as f64;
        return round(perc, 3);
    }
}

// TODO: move this to separate file
fn round(target: f64, precision: u32) -> f64 {
    let r = u32::pow(10, precision) as f64;
    return (target * r).round() / r;
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, NaiveDateTime, DateTime};

    use crate::{TimeLeft, TimeLeftTime, TimeWindow};

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

    #[test]
    fn given_arbitrary_time_of_day_when_get_day_left_then_returns_correct() {
        let expected = 0.436;

        let actual = TimeLeft::new(create_time_from("2023-01-01 13:31:45")).get_day_left();

        assert_eq!(expected, actual);
    }

    #[test]
    fn given_point_as_half_when_get_pecentage_then_returns_half() {
        let expected = 0.5;
        let mut tw = TimeWindow::new(create_time_from("2023-01-01 12:00:00"), create_time_from("2023-01-01 13:00:00"));

        tw.set_point(create_time_from("2023-01-01 12:30:00"));
        let actual = tw.get_percentage();

        assert_eq!(expected, actual);
    }

    #[test]
    fn given_point_as_t1_when_get_pecentage_then_returns_0() {
        let expected = 0.0;
        let mut tw = TimeWindow::new(create_time_from("2023-01-01 12:00:00"), create_time_from("2023-01-01 13:00:00"));

        tw.set_point(create_time_from("2023-01-01 12:00:00"));
        let actual = tw.get_percentage();

        assert_eq!(expected, actual);
    }

    #[test]
    fn given_point_as_t2_when_get_pecentage_then_returns_1() {
        let expected = 1.0;
        let mut tw = TimeWindow::new(create_time_from("2023-01-01 12:00:00"), create_time_from("2023-01-01 13:00:00"));

        tw.set_point(create_time_from("2023-01-01 13:00:00"));
        let actual = tw.get_percentage();

        assert_eq!(expected, actual);
    }
}

fn main() {
    let tl = TimeLeft::default();
    println!("Hello, world! {0}", tl.get_day_left());
}
