
extern crate csv;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn read_csv_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    for result in rdr.records() {
        let record = result?;
        for field in record.iter() {
            print!("{}, ", field);
        }
        println!();
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_csv_file("data.csv") {
        eprintln!("error reading CSV file: {}", err);
    }
}


