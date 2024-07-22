use chrono::{NaiveDateTime, Timelike};

fn main() {
    let date_time_str = "2024-07-22 08:19:07";
    let format_str = "%Y-%m-%d %H:%M:%S";

    // Parse the input date-time string
    let parsed_date_time = NaiveDateTime::parse_from_str(date_time_str, format_str)
        .expect("Failed to parse date-time string");

    // Calculate the Unix timestamp
    let unix_timestamp = parsed_date_time.timestamp();

    println!("Unix timestamp: {}", unix_timestamp);
}
