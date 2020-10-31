#[derive(Debug)]
struct Circle {
  radius: f64,
}
#[derive(Debug)]
struct Square {
  side: f64,
}

trait Area {
  fn area(&self) -> f64;
}

impl Area for Circle {
  fn area(&self) -> f64 {
    self.radius.powf(self.radius) * std::f64::consts::PI
  }
}

impl Area for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }
}

// Calls are made at run time
pub fn dynamic() {
  let shapes: [&Area; 4] = [
    &Circle { radius: 1.0 },
    &Square { side: 3.0 },
    &Circle { radius: 2.0 },
    &Square { side: 8.0 },
  ];

  for (i, shape) in shapes.iter().enumerate() {
    println!("{:?}, {:?}", shape.area(), shape.area());
  }
}
