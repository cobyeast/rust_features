fn vec() -> Vec<[i32; 4]> {
  let mut a = Vec::new();
  let u: usize = 1;

  let b: [i32; 4] = [10; 4];
  let c: [i32; 4] = [15; 4];

  a.push(b);
  a.push(c);

  match a.get(2) {
    Some(x) => println!("x = {:?}", x),
    None => print!("None\n"),
  }

  // for (i, x) in a.iter().enumerate() {
  //   println!("{}", x[i]);
  // }

  // let last = a.pop();

  while let Some(x) = a.pop() {
    println!("{:?}", x)
  }

  a
}

pub fn vectors() {
  vec();
  // match vec().pop() {
  //   Some(x) => println!("{:?}", x),
  //   None => (),
  // }
}
