use std::fmt::Debug;

#[derive(Debug)] // impl the debug trait
struct Circle {
  radius: f64,
}

trait Shape {
  fn area(&self) -> f64;
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    self.radius * self.radius * std::f64::consts::PI
  }
}

// Argument shape implements two different traits (Shape, Debug)
fn print_info<T: Shape + Debug>(shape: T) {
  println!("{:?}", shape);
  println!("area = {}", shape.area());
}

pub fn traits() {
  let c = Circle { radius: 8.0 };
  print_info(c);
}
