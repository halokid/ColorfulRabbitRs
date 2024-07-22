use chrono::{NaiveDateTime, ParseResult};
use crate::CustomErr;

pub fn convert_datet_to_unixt(datet: String) -> Result<i64, CustomErr> {
  let format_str = "%Y-%m-%d %H:%M:%S";

  let parsed_date_time = NaiveDateTime::parse_from_str(date_time_str, format_str);
  match parsed_date_time {
    Ok(parsed_date_time) => {
      let unix_timestamp = parsed_date_time.timestamp();
      Ok(unix_timestamp)
    }
    Err(_) => {
      return Err(CustomErr("Failed to parse date-time string".to_string()));
    }
  }
}