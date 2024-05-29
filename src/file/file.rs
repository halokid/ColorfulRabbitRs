use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use crate::CustomErr;

// trait ForLine {
//   fn linestr() -> String;
// }
//
// impl ForLine for T {
//   fn linestr() -> String {
//     "line".to_string()
//   }
// }

// pub fn write<T: Debug>(filepath: String, data: Vec<T>) -> Result<(), CustomErr> {
pub fn write<T: Debug>(filepath: String, data: Vec<T>) -> Result<(), CustomErr> {

  // trait ForLine {
  //   fn linestr() -> String;
  // }

  // impl ForLine for T {
  //   fn linestr() -> String {
  //     "line".to_string()
  //   }
  // }

  let mut file = File::create(filepath).unwrap();
  for item in data {
    println!("item -->>> {:?}", item);

    // let line = format!("{}\n", item.linestr());
    // let lineb = line.as_bytes();
    let lineb = b"hello\n";
    // write!(filepath, line.as_str()).unwrap();
    file.write(lineb).unwrap();
  }

  Ok(())
}



