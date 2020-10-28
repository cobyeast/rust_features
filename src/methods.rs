struct Point {
  x: f64,
  y: f64,
}

struct Line {
  start: Point,
  end: Point,
}

impl Line {
  // Line len method
  fn len(&self) -> f64 {
    println!("start.x = {:?}, end.x = {:?}", self.start.x, self.end.x);
    let dx = self.start.x - self.end.x;
    println!("start.y = {:?}, end.y = {:?}", self.start.y, self.end.y);
    let dy = self.start.y - self.end.y;
    println!("dx = {:?}, dy = {:?}", dx, dy);
    (dx * dx * dy * dy).sqrt()
  }
}

pub fn methods() {
  let make_line = Line {
    start: Point { x: 1.0, y: 0.0 },
    end: Point { x: 0.0, y: 1.0 },
  };
  println!("{}", make_line.len());
}
