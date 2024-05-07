
use colorful_rabbit_rs::file::xls_csv;

#[test]
fn test_read_csv() {
  let file_path = "./data.csv";
  let lines = xls_csv::read_csv(file_path);
  println!("{:?}", lines.unwrap());
}

#[test]
fn test_write_xls() {
  let file_path = "./data.csv";
  let lines = xls_csv::read_csv(file_path);
  let content = lines.unwrap();
  println!("{:?}", content);

  let w_file_path = "./demo.xlsx";
  let w = xls_csv::write_xls(w_file_path, content);
  println!("{:?}", w);
}



