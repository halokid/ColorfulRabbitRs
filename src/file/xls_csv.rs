use rust_xlsxwriter::*;
use calamine::{open_workbook, DataType, Error, Reader, Xlsx};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::task::Poll::Pending;
use csv::ReaderBuilder;
use crate::CustomErr;

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<String>>, CustomErr> {
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

pub fn read_text_csv(file_path: &str) -> Result<Vec<Vec<String>>, CustomErr> {
  let file = File::open(file_path).unwrap();
  let lines = BufReader::new(file).lines();

  let mut res_lines: Vec<Vec<String>> = Vec::new();
  let mut i = 0;
  for line in lines {
    // match line {
    //   Ok(_) => {}
    //   Err(_) => {}
    // }
    if let Ok(data) = line {
      if i > 100 {
        break;
      }
      println!("data -->>> {}", data);
      let line_split = data.split(",");
      // let res: Vec<String> = line_split.collect();
      let res: Vec<String> = line_split.map(|s| s.to_string()).collect();
      res_lines.push(res);
      i += 1;
    }
  }
  Ok(res_lines)
}

pub fn write_xls(file_path: &str, content: Vec<Vec<String>>) -> Result<(), CustomErr> {
  // Create a new Excel file object.
  let mut workbook = Workbook::new();
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

pub fn read_excel(file_path: &str, sheet: &str) -> Result<(), CustomErr> {
  // let file_path = "test.xlsx";

  // opens a new workbook
  let mut workbook: Xlsx<_> = open_workbook(file_path).expect("Cannot open file");

  let worksheet = workbook
    .worksheet_range(sheet)
    .expect("Cannot find 'Sheet1'");

  // 遍历工作表中的所有行
  let mut i = 0;
  for row in worksheet.rows() {
    // if i > 100 {
    //   break
    // }
    let mut vec_row_data: Vec<String> = Vec::new();
    // 遍历当前行的所有单元格
    for cell in row {
      // 将单元格的值转换为字符串并添加到row_data向量中
      vec_row_data.push(cell.to_string());
    }
    //打印
    println!("{:?}, x: {}, y: {}", vec_row_data, vec_row_data.get(20).unwrap(), vec_row_data.get(2).unwrap());
    i += 1;
  }

  // 关闭工作簿
  drop(workbook);

  Ok(())
}

