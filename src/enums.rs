type RefStr = &'static str;

pub enum FarmAnimals {
  Dog,
  Sheep(RefStr, RefStr, RefStr), // Tuple
  Pig,
  Goat { milk: bool }, // Struct
  Cow { milk: bool },  // Struct
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
