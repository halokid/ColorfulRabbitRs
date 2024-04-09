use rust_xlsxwriter::*;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::CustomErr;

pub fn read_csv(file_path: &str) -> Result<Vec<String>, CustomErr> {
  let file = File::open(file_path).unwrap();
  let mut rdr = ReaderBuilder::new().from_reader(file);

  let mut res = Vec::new();
  for result in rdr.records() {
    let record = result.unwrap();
    let line = record.as_slice();
    res.push(line.to_string());
    // println!();
  }
  Ok(res)
}

