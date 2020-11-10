#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
}

impl<'a> Person<'a> {
  fn greeting(&self) {
    println!("{}", self.name);
  }
}

fn lifetime() {
  let p = Person { name: "Joe" };
  p.greeting();
}

pub fn lifetimes() {
  lifetime();
}
