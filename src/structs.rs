// #[derive(Copy, Clone)]
struct Point {
  x: f64,
  y: f64,
}

impl Copy for Point {}

impl Clone for Point {
  fn clone(&self) -> Point {
    *self
  }
}

struct Line {
  start: Point,
  end: Point,
}

pub fn structures() {
  let p = Point { x: 1.0, y: 3.0 };
  let p2 = Line {
    start: Point { x: 5.0, y: 10.0 },
    end: p,
  };
}
