use std::sync::Arc;

struct Person {
  name: Arc<String>
}

impl Person {
  fn new(name: Arc<String>) -> Person {
    Person { name }
  }
  fn greet(&self) {
    println!("Hello, {}!", self.name);
  }
}

pub fn arc(){
  let name = Arc::new("Mazzy".to_string());
  let p = Person::new(name.clone());

  let t = std::thread::spawn(move || {
    p.greet();
  });

  t.join().unwrap();
}