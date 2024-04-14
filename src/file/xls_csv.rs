use rust_xlsxwriter::*;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::CustomErr;

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<String>>,
  CustomErr> {
  let file = File::open(file_path).unwrap();
  // let mut rdr = ReaderBuilder::new().from_reader(file);
  let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

  // let mut res = Vec::new();
  let mut lines: Vec<Vec<String>> = Vec::new();
  for result in rdr.records() {
    let record = result.unwrap();
    let mut line = Vec::new();
    for field in record.iter() {
      line.push(field.to_string());
    }
    lines.push(line);
  }
  Ok(lines)
}

