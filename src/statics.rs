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

// Expensive call
fn print_expense(z: &Printable) {
  z.format();
}

// Inexpensive call
fn print_inexpense<T: Printable + Display>(z: T) {
  z.format();
}

pub fn statics() {
  let a = 123;
  let b = "hello".to_string();

  print_inexpense(a);
  print_inexpense(b);
}
