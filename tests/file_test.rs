
use colorful_rabbit_rs::file::file;

#[derive(Debug)]
struct User {
  name: String,
  location:  String,
}

impl User {

  pub fn linestr(&self) -> String {
    "hello".to_string()
  }
}

#[test]
fn test_file_write() {
  let file_path = "./data.txt".to_string();

  let u1 = User{ name: "aaa".to_string(), location: "111".to_string() };
  let u2 = User{ name: "bbb".to_string(), location: "222".to_string() };
  let u3 = User{ name: "ccc".to_string(), location: "333".to_string() };
  let mut data = Vec::new();
  data.push(u1);
  data.push(u2);
  data.push(u3);

  file::write(file_path, data).expect("TODO: panic message");
}
