use std::io;
use std::io::BufRead;
use std::process::{Command, Stdio};

pub fn runcmd(cmd: &str, args: &[&str]) -> String {
  let output = Command::new(cmd)
    // .arg("-c")
    // .arg("echo hello")
    // .args(["-l", "-a"])
    .args(args)
    .output()
    .expect("failed to execute process");
  let run_res = output.stdout;
  String::from_utf8(run_res).unwrap()
}

pub fn runcmd_pipe(cmd: &str, args: &[&str]) {
  let mut command = Command::new(cmd);
  command.args(args);
  command.stdout(Stdio::piped()); // 将stdout管道化以便获取输出

  // start command and get it's stdout
  let child = command.spawn().unwrap();
  let stdout = child.stdout.expect("Failed to open stdout");

  // use BufReader read output line by line
  let reader = io::BufReader::new(stdout);
  for line in reader.lines() {
    println!("{}", line.unwrap());
  }
}