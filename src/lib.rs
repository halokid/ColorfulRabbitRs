use std::fmt;

pub mod utils;
pub mod file;
pub mod datet;

#[derive(Debug)]
pub struct CustomErr (pub String);

impl fmt::Display for CustomErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "CustomErr -->>> {}", self.0)
  }
}

