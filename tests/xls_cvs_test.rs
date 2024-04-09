
use colorful_rabbit_rs::file::xls_csv;

#[test]
fn test_read_csv() {
  let file_path = "./data.csv";
  let lines = xls_csv::read_csv(file_path);
  println!("{:?}", lines.unwrap());
}



