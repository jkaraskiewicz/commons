use chrono::{DateTime, Local};
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns current date in the format of YYYYmmdd
pub fn current_formatted_date() -> String {
    let current_datetime: DateTime<Local> = Local::now();
    current_datetime.format("%Y%m%d").to_string()
}

/// Returns current timestamp
pub fn current_timestamp() -> u64 {
    let current_datetime: DateTime<Local> = Local::now();
    current_datetime.timestamp() as u64
}

/// Returns timestamp for a given SystemTime
pub fn timestamp_for_systemtime(system_time: &SystemTime) -> u64 {
    system_time.duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// Formats a datetime passed as a SystemTime in the format of YYYY-mm-dd HH:MM
pub fn formatted_systemtime(time: &SystemTime) -> String {
    let datetime: DateTime<Local> = time.to_owned().into();
    datetime.format("%Y-%m-%d %H:%M").to_string()
}
