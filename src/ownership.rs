fn own() {
  let v = vec![2, 3, 4];

  {
    let v2 = v;
    let prnt = |x: Vec<i32>| -> Vec<i32> {
      println!("{:?}", x);
      x
    };
    let v3 = prnt(v2);
    println!("{:?}", v3[0]);
  }

  let u = Box::<u8>::new(6);
  let u2 = u;
  println!("{:?}", *u2);
}

pub fn ownership() {
  own();
}
