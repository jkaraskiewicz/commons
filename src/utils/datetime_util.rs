use chrono::{DateTime, Local};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_formatted_date() -> String {
    let current_datetime: DateTime<Local> = Local::now();
    current_datetime.format("%Y%m%d").to_string()
}

pub fn current_timestamp() -> u64 {
    let current_datetime: DateTime<Local> = Local::now();
    current_datetime.timestamp() as u64
}

pub fn timestamp_for_systemtime(system_time: &SystemTime) -> u64 {
    system_time.duration_since(UNIX_EPOCH).unwrap().as_secs()
}
