use std::convert::From;
use std::fmt::Debug;

#[derive(Debug)]
struct Person {
  name: String,
}

impl Person {
  // fn new(name: &str) -> Person {
  //   Person {name:name.to_string()},
  // }
  fn new<T: Into<String> + Debug>(name: T) -> Person {
    Person { name: name.into() }
  }
}

pub fn intos() {
  // let john = Person::new("John");
  // let name: String = "Jane".to_string();
  let num: String = Person::new(2i32);

  println!("{:?}", num);
}
