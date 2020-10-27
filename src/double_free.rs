pub fn twice<F>(call: F) -> String
where
  F: FnOnce(),
{
  call();
  "Dropped".to_string()
}

pub fn do_this() -> String {
  "hi".to_string()
}

pub fn string_maker() {
  let my_str = "hi".to_string();
  let free = || drop(my_str);
  twice(free);
}
