use rust_xlsxwriter::*;

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

pub fn write_xls(file_path: &str, content: Vec<Vec<String>>) -> Result<(), CustomErr> {
  // Create a new Excel file object.
  let mut workbook =  Workbook::new();
  // Add a worksheet to the workbook.
  let worksheet = workbook.add_worksheet();

  let column_len = content[0].len() as f64;
  worksheet.set_column_width(0, column_len).unwrap();

  let mut i = 0;
  for line in content.iter() {
    let mut j = 0;
    for col in line.iter() {
      worksheet.write(i, j, col).unwrap();
      j += 1;
    }
    i += 1;
  }

  // Save the file to disk.
  workbook.save(file_path).unwrap();

  Ok(())
}


