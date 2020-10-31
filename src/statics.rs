use std::fmt::Display;

trait Printable {
  fn format(&self) -> String;
}

impl Printable for i32 {
  fn format(&self) -> String {
    format!("{}", *self)
  }
}

impl Printable for String {
  fn format(&self) -> String {
    format!("string: {}", *self)
  }
}

fn print_it<T: Printable + Display>(z: T) {
  z.format();
}

pub fn statics() {
  let a = 123;
  let b = "hello".to_string();

  print_it(a);
  print_it(b);
}
