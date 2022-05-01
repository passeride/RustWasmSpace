use chrono::prelude::{DateTime, Datelike, NaiveDateTime, Utc};

pub struct UniversalTime {
    pub date: f64,
    pub datetime: DateTime<Utc>,
}

impl UniversalTime {
    pub fn from_iso8601(time: &DateTime<Utc>) -> UniversalTime {
        let y = time.year() as f64;
        let m = time.month() as f64;
        let d = time.day() as f64;
        let d: f64 =
            367.0 * y - 7.0 * (y + (m + 9.0) / 12.0) / 4.0 + 275.0 * m / 9.0 + d - 730530.0;

        UniversalTime {
            date: d,
            datetime: time.clone(),
        }
    }

    pub fn to_iso_string(&self) -> String {
        format!("{}", self.datetime.format("%+"))
    }

    pub fn iso8601(st: &std::time::SystemTime) -> String {
        let dt: DateTime<Utc> = st.clone().into();
        format!("{}", dt.format("%+"))
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::{DateTime, Datelike, NaiveDateTime, Utc};
    use std::time::SystemTime;

    use super::UniversalTime;

    #[test]
    fn universal_time() {
        // let now = SystemTime::now();
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(946684800, 0), Utc);
        let ut = UniversalTime::from_iso8601(&dt);
        print!("Now datetime is {}", ut.date);
        print!("Now datetime is {}", ut.to_iso_string());
    }

    #[test]
    fn now_to_iso8601() {
        let now = SystemTime::now();
        print!("Now Time{}\n", UniversalTime::iso8601(&now));
    }
}
