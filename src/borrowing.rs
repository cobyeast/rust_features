fn borrow() {
  let prnt = |x: &Vec<u8>| {
    println!("{}", x[0]);
  };

  let v = vec![9, 8, 7];
  prnt(&v);

  println!("{}", v[0]);

  let mut a = 40;

  // Create inner scope
  {
    let b = &mut a;
    *b += 10;
  }

  println!("a = {}", a);

  let mut z = vec![7, 6, 5];

  // for i in &z {
  //   z.push(5); // unable to push (rust feature)
  // }
}

pub fn borrowing() {
  borrow();
}
