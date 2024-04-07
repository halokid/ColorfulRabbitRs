use std::io;
use std::io::BufRead;
use std::process::{Command, Stdio};

fn c1() {
  let output = Command::new("sh")
    .arg("-c")
    .arg("echo hello")
    .output()
    .expect("failed to execute process");

  println!("command output -->>> {:?}", output.stdout);
  println!("command output -->>> {:?}", String::from_utf8(output.stdout));

  let output2 = Command::new("ls")
    .arg("-l")
    .output()
    .expect("failed to execute process");
  println!("command output -->>> {:?}", String::from_utf8(output2.stdout));
}

fn c2() -> io::Result<()> {
    // 创建一个Command实例来执行命令
    // let mut command = Command::new("ls");
    // command.arg("-l"); // 添加命令参数

    let mut command = Command::new("sh");
    command.arg("./test.sh"); // 添加命令参数
    command.stdout(Stdio::piped()); // 将stdout管道化以便获取输出

    // 启动命令并获取其stdout
    let child = command.spawn()?;
    let stdout = child.stdout.expect("Failed to open stdout");

    // 使用BufReader逐行读取输出
    let reader = io::BufReader::new(stdout);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
  // c1();

  c2();
}