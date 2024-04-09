extern crate csv;

use rust_xlsxwriter::*;
use colorful_rabbit_rs::file::xls_csv;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn read_csv_file(file_path: &str) -> Result<(), Box<dyn Error>> {
  let file = File::open(file_path)?;
  // let mut rdr = ReaderBuilder::new().from_reader(file);
  let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

  // for result in rdr.records() {
  //   let record = result?;
  //   for field in record.iter() {
  //     print!("{}, ", field);
  //   }
  //   println!();
  // }

  let mut lines: Vec<Vec<String>> = Vec::new();
  for result in rdr.records() {
    let record = result?;
    let mut line = Vec::new();
    for field in record.iter() {
      // line.push(field);
      line.push(field.to_string());
      // print!("{}, ", field);
    }
    println!("{:?}", line);
    lines.push(line);
  }

  // for result in rdr.records() {
  //   let record = result?;
  //   let line = record.as_slice();
  //   println!("{}", line);
  // }
  println!("lines len: {}", lines.len());
  Ok(())
}

fn write_xls() -> Result<(), XlsxError> {
  // Create a new Excel file object.
  let mut workbook = Workbook::new();

  // Create some formats to use in the worksheet.
  let bold_format = Format::new().set_bold();
  let decimal_format = Format::new().set_num_format("0.000");
  let date_format = Format::new().set_num_format("yyyy-mm-dd");
  let merge_format = Format::new()
    .set_border(FormatBorder::Thin)
    .set_align(FormatAlign::Center);

  // Add a worksheet to the workbook.
  let worksheet = workbook.add_worksheet();

  // Set the column width for clarity.
  worksheet.set_column_width(0, 22)?;

  // Write a string without formatting.
  worksheet.write(0, 0, "Hello")?;

  // Write a string with the bold format defined above.
  worksheet.write_with_format(1, 0, "World", &bold_format)?;

  // Write some numbers.
  worksheet.write(2, 0, 1)?;
  worksheet.write(3, 0, 2.34)?;

  // Write a number with formatting.
  worksheet.write_with_format(4, 0, 3.00, &decimal_format)?;

  // Write a formula.
  worksheet.write(5, 0, Formula::new("=SIN(PI()/4)"))?;

  // Write a date.
  let date = ExcelDateTime::from_ymd(2023, 1, 25)?;
  worksheet.write_with_format(6, 0, &date, &date_format)?;

  // Write some links.
  worksheet.write(7, 0, Url::new("https://www.rust-lang.org"))?;
  worksheet.write(8, 0, Url::new("https://www.rust-lang.org").set_text("Rust"))?;

  // Write some merged cells.
  worksheet.merge_range(9, 0, 9, 1, "Merged cells", &merge_format)?;

  worksheet.write(1, 2, "test xx")?;

  // Insert an image.
  // let image = Image::new("examples/rust_logo.png")?;
  // worksheet.insert_image(1, 2, &image)?;

  // Save the file to disk.
  workbook.save("demo.xlsx")?;

  Ok(())
}

fn read_csv() {
  let lines = xls_csv::read_csv("data.csv").unwrap();
  println!("{:?}", lines);
}

fn main() {
  if let Err(err) = read_csv_file("data.csv") {
    eprintln!("error reading CSV file: {}", err);
  }

  // write_xls();

  // read_csv();
}




