fn add_ten(v: &mut i32) -> &mut i32 {
  *v += 10;
  v
}

pub fn functions() {
  let mut val: i32 = 1;
  for i in 1..10 {
    println!("{}", add_ten(add_ten(&mut val)));
  }
}
