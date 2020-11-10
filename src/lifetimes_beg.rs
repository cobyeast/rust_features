struct Person {
  name: String,
}

impl Person {
  fn get_name<'a>(&'a self) -> &'a String {
    &self.name
  }
}

struct Company<'z> {
  name: String,
  ceo: &'z Person, // 'z lifetime allows a reference to Person and ensures we can't go out of scope
}

fn lifetime() {
  // Note: &'static str // 'static a special lifetime lives as long as program

  let boss = Person {
    name: String::from("Johnny Mark"),
  };

  let cloudflare = Company {
    name: String::from("Cloudflare"),
    ceo: &boss,
  };

  println!("{}", boss.get_name());

  let z: &String;

  {
    let p = Person {
      name: String::from("John"),
    };

    z = p.get_name();

    println!("{}", z);
  }
}

pub fn lifetimes() {
  lifetime();
}
