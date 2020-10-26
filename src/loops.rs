pub fn loops() {
  let mut x = 1;

  loop {
    x *= 2;
    println!("x = {}", x);
    if x == 1 << 10 {
      break;
    }
  }

  for i in 1..11 {
    println!("i = {}", i);
    if i == 3 {
      break;
    }
  }

  for (pos, y) in (30..41).enumerate() {
    println!("{}: {}", pos, y)
  }
}
