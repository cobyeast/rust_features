use std::rc::Rc;

struct Person {
  name: Rc<String>,
}

impl Person {
  fn new(name: Rc<String>) -> Self {
    Self { name }
  }
  fn greet(&self) {
    print!("Hello, my name is {}!\n", self.name);
  }
}

pub fn rc() {
  let p_name: Rc<String> = Rc::new("Timmy Turner".to_string());
  print!(
    "p_name = {}, name has {} strong pointers\n",
    p_name,
    Rc::strong_count(&p_name)
  );
  // Create scope
  {
    let p = Person::new(p_name.clone());
    let p = Person::new(p_name.clone()); // Call clone to increment reference count with out use of a 'move'
    p.greet();
    print!(
      "p_name = {}, name has {} strong pointers\n",
      p_name,
      Rc::strong_count(&p_name)
    );
  }
  print!(
    "p_name = {}, name has {} strong pointers\n",
    p_name,
    Rc::strong_count(&p_name)
  )
}
