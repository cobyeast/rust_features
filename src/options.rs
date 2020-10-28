pub fn options() {
  let mut time: Vec<u8> = vec![];
  let x = 1.0;
  let y = 5.0;

  // Option -> Some(v) | None
  let result = if y != 0.0 { Some(x / y) } else { None };

  match result {
    Some(z) => print!(" "),
    None => (),
  }

  if let Some(z) = result {
    println!("result = {}", z);
  }

  while let Some(z) = result {
    // Do nothing!!
  }

  while time.len() < 10 {
    println!("x = {}, y = {}", x, y);
    time.push(0);
  }
}
