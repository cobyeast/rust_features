#[allow(unused_variables)]

union IntOrFloat {
  i: i32,
  f: f32,
}

fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat { i: 100 } => {
        println!("{}", "It is One Hundred!");
      }
      IntOrFloat { f } => {
        println!("{}", f);
      }
    }
  }
}

pub fn unions() {
  let mut iof = IntOrFloat { i: 120 };
  iof.i = 100;

  let value = unsafe { iof.i };

  process_value(IntOrFloat { i: 10 });
}
