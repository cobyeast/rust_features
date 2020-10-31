use std::ops::{Add, AddAssign, Neg};

#[derive(Debug, Eq)]
struct Complex<T> {
  re: T,
  im: T,
}

impl<T> Complex<T> {
  fn new(re: T, im: T) -> Complex<T> {
    Complex::<T> { re, im }
  }
}

// implement an Add trait that accepts multiple types
impl<T: Add<Output = T>> Add for Complex<T> {
  type Output = Complex<T>;

  fn add(self, rhs: Self) -> Self::Output {
    Complex {
      re: self.re + rhs.re,
      im: self.im + rhs.im,
    }
  }
}

impl<T: AddAssign> AddAssign for Complex<T> {
  fn add_assign(&mut self, rhs: Self) {
    self.re += rhs.re;
    self.im += rhs.im;
  }
}

impl<T: Neg<Output = T>> Neg for Complex<T> {
  type Output = Complex<T>;
  fn neg(self) -> Self::Output {
    Complex {
      re: -self.re,
      im: -self.im,
    }
  }
}

impl<T: PartialEq> PartialEq for Complex<T> {
  fn eq(&self, rhs: &Self) -> bool {
    self.re == rhs.re && self.im == rhs.im
  }
}

pub fn overload() {
  let mut a = Complex::new(1.2, 2.2);
  let mut b = Complex::new(1.2, 2.2);

  // a += b;

  println!("a = b {:?}", a == b);
}
