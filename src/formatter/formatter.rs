//  Formatter for the IR
//  Copyright (c) 2024 By Yazalde Filimone
//  MIT License

pub struct Formatter {
  pub indent: usize,
  pub newline: bool,
  pub ir_string: String,
}

impl Formatter {
  pub fn new() -> Self {
    Self { indent: 0, newline: false, ir_string: String::new() }
  }

  pub fn format(&self) {
    todo!("Format IR");
  }
}
