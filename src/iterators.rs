fn iter_1() {
  let mut vec = vec![5, 4, 3];

  // vec.iter() is a immutable version of iter()
  for i in vec.iter() {
    println!("{}", i);
  }

  // vec.iter_mut() is a mutable version of iter() (altering of values)
  for i in vec.iter_mut() {
    *i += 5
  }
  println!("{:?}", vec);
}

fn iter_2() {
  let mut vec1 = vec![1, 2, 3];
  let mut vec2 = vec![3, 2, 1];

  // Extend runs vec1.into_iter();
  // Note: after call vec2 is un-usable
  vec1.extend(vec2);

  println!("{:?}", vec1);
}

pub fn iterators() {
  iter_2()
}
