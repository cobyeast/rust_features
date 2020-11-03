pub trait Animal {
  fn create(name: &'static str) -> Self;
  fn name(&self) -> &'static str;
  fn talk(&self) {
    println!("{} can not talk", self.name());
  }
}

pub struct Human {
  name: &'static str,
}

pub struct Cat {
  name: &'static str,
}

impl Animal for Human {
  fn create(name: &'static str) -> Human {
    Human { name: name }
  }
  fn name(&self) -> &'static str {
    self.name
  }
  fn talk(&self) {
    println!("{} says hello", self.name());
  }
}

impl Animal for Cat {
  fn create(name: &'static str) -> Cat {
    Cat { name: name }
  }
  fn name(&self) -> &'static str {
    self.name
  }
  fn talk(&self) {
    println!("{} says meow", self.name());
  }
}

// Trait with a custom <T>
trait Summable<T> {
  fn sum(&self) -> T;
}

// Implement a method for an existing trait Vec<i32>
impl Summable<i32> for Vec<i32> {
  fn sum(&self) -> i32 {
    let mut result: i32 = 0;
    for i in self {
      result += *i;
    }
    result
  }
}

// Implement a method for an existing trait Vec<u8>
pub impl Summable<u8> for Vec<u8> {
  fn sum(&self) -> u8 {
    let mut result: u8 = 0;
    for i in self {
      result += *i;
    }
    result
  }
}

pub fn traits() {
  let h = Human { name: "Jim" };
  println!("human = {}", h.name());
  h.talk();

  let c: Cat = Animal::create("Sprinkles");
  println!("cat = {}", c.name());
  c.talk();

  let a = vec![1, 2, 3, 4];
  println!("sum = {}", a.sum());

  let b: Vec<u8> = vec![1, 2, 4, 5];
  println!("sum = {}", b.sum());
}
