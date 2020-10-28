fn use_slices(slice: &mut [i32]) -> &mut [i32] {
  slice[3] = 1020;
  &mut slice[3..4]
}

pub fn slices() {
  let mut array = [1, 2, 3, 4, 5];
  println!("{:?}", use_slices(&mut array));
}
