// Double Free

pub fn twice<F>(call: F) -> String
where
  F: FnOnce(),
{
  call();
  // call();
  "Dropped".to_string()
}

fn make_str() -> String {
  "hi".to_string()
}

pub fn double_free() {
  let my_str = "hi".to_string();
  let free = || drop(my_str);
  twice(free);
}

// References and Memory Allocation

fn local(s: &String) {
  println!("{}", s);
}

pub fn references() {
  let c = String::from("Hello");
  println!("{:?}", a);
  local(&c);
  local(&c);
}
