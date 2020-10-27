type RefStr = &'static str;

pub enum FarmAnimals {
  Dog,
  Sheep(RefStr, RefStr, RefStr),
  Pig,
  Goat { milk: bool },
  Cow { milk: bool },
}

pub fn farm() {
  let animal: FarmAnimals = FarmAnimals::Sheep("", "", "gray");
  match animal {
    FarmAnimals::Dog => {
      println!("{}", "Dog");
    }
    FarmAnimals::Sheep(_, _, "gray") => {
      println!("{}", "Gray Sheep");
    }
    FarmAnimals::Goat { milk: true } => {
      println!("{}", "Milk Goat");
    }
    _ => (),
  }
}
