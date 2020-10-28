#[allow(unused_variables)]

fn local(x: u16) -> u16 {
  x + 9_999
}

pub fn closures() {
  let l = local;
  let inner_one = |x: u16| -> u16 { x + 1 };
  let mut two: u16 = 2;
  {
    let inner_two = |x: u16| {
      let mut z = inner_one(x);
      z += two;
      z
    };
    println!("{}", inner_two(l(two)));
    let borrow_two = &mut two;

    // Passed Copy
    let inner_three = |mut x: i32| x *= 3;

    // Passed Reference
    let inner_four = |x: &mut i32| *x += 3;

    let mut f: i32 = 12;
    inner_three(f);
    inner_four(&mut f);

    println!("{}", f);
  }
}
