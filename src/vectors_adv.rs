trait Animal {
  // fn create(name: &'static str) -> Self;
  fn name(&self) -> &'static str;
  fn talk(&self) {
    println!("{} can not talk", self.name());
  }
}

#[derive(Debug)]
pub struct Human {
  name: &'static str,
}

#[derive(Debug)]
pub struct Cat {
  name: &'static str,
}

impl Animal for Human {
  // fn create(name: &'static str) -> Human {
  //   Human { name: name }
  // }
  fn name(&self) -> &'static str {
    self.name
  }
  fn talk(&self) {
    println!("{} says hello", self.name());
  }
}

impl Animal for Cat {
  // fn create(name: &'static str) -> Cat {
  //   Cat { name: name }
  // }
  fn name(&self) -> &'static str {
    self.name
  }
  fn talk(&self) {
    println!("{} says meow", self.name());
  }
}

#[derive(Debug)]
enum Creature {
  Human(Human),
  Cat(Cat),
}

fn vec() {
  let mut creatures: Vec<Creature> = Vec::<Creature>::new();
  creatures.push(Creature::Human(Human { name: "John" }));
  creatures.push(Creature::Cat(Cat { name: "Fluffy" }));

  // for c in creatures {
  //   match c {
  //     Creature::Human(h) => h.talk(),
  //     Creature::Cat(c) => c.talk(),
  //   }
  // }

  let mut animals: Vec<Box<Animal>> = Vec::<Box<Animal>>::new();
  animals.push(Box::new(Human { name: "Jerry" }));
  animals.push(Box::new(Cat { name: "Tiger" }));

  for a in animals.iter() {
    a.talk();
  }
}

pub fn vectors() {
  vec();
}
