use std::sync::{Arc, Mutex};

struct Person {
  name: Arc<String>,
  state: Arc<Mutex<String>>
}

impl Person {
  fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
    Person { name, state }
  }
  fn greet(&self) {
    let mut state = self.state.lock().unwrap();

    state.clear();
    state.push_str("excited");

    println!("Hello, {}! Are you {}?", self.name, state.as_str());
  }
}

pub fn mutex(){
  let name = Arc::new("Mazzy".to_string());
  let state = Arc::new(Mutex::new("bored".to_string()));
  let p = Person::new(name.clone(), state.clone());

  let t = std::thread::spawn(move || {
    p.greet();
  });

  println!("name = {}, state = {}", name, state.lock().unwrap().as_str());

  t.join().unwrap();
}