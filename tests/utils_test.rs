
use colorful_rabbit_rs::utils::utils;

#[test]
fn test_runcmd() {
  // let strings = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];

  // let strings = ["apple", "banana", "cherry", "date"];
  // let sl: &[&str] = &strings;
  let args = ["-l", "-a",];
  let res = utils::runcmd("ls", &args);

  let args = ["./test.sh"];
  let res = utils::runcmd("sh", &args);
  println!("res -->>> {:?}", res);

  let args = ["./test.sh"];
  let res = utils::runcmd_pipe("sh", &args);
}