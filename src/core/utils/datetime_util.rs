use chrono::{DateTime, Utc};

pub struct DateTimeUtil;

impl DateTimeUtil {
    pub fn date_at_midnight(date_time: &DateTime<Utc>) -> DateTime<Utc> {
        date_time
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    }
}
