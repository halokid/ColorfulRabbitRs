use std::fs;
use std::path::Path;
use chrono::{NaiveDateTime, Timelike};

fn main() {
  let root_folder = Path::new("./tests");

  let latest_folder = fs::read_dir(root_folder)
    .expect("Failed to read directory")
    .filter_map(|entry| {
      entry.ok().and_then(|e| {
        e.metadata().ok().map(|m| (e.file_name(), m.modified().ok()))
      })
    })
    .filter_map(|(name, modified)| modified.map(|m| (name, m)))
    .max_by_key(|&(_, time)| time)
    .map(|(name, _)| name)
    .map(|name| name.to_string_lossy().into_owned());

  match latest_folder {
    Some(folder_name) => println!("Latest folder: {}", folder_name),
    None => println!("No folders found"),
  }

  // ----------------------------------------------------
  let date_time_str = "2024-07-22 08:19:07";
  let format_str = "%Y-%m-%d %H:%M:%S";

  // Parse the input date-time string
  let parsed_date_time = NaiveDateTime::parse_from_str(date_time_str, format_str)
    .expect("Failed to parse date-time string");

  // Calculate the Unix timestamp
  let unix_timestamp = parsed_date_time.timestamp();

  println!("Unix timestamp: {}", unix_timestamp);
}


