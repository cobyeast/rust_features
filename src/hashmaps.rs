use std::collections::HashMap;

fn hashmap() {
  let mut shapes = HashMap::new();

  shapes.insert(String::from("triangle"), 3);
  shapes.insert(String::from("square"), 4);

  shapes.insert("square".into(), 5);
  shapes.entry("circle".into()).or_insert(1);

  {
    // New Scope For Borrowing Value
    let actual = shapes.entry("circle".into()).or_insert(3); // Reference
    *actual = 0; // Dereference and Change
  }

  for (key, val) in shapes {
    print!("{}: {}\n", key, val);
  }
}

pub fn hashmaps() {
  hashmap();
}
