pub fn arrays() {
  // Mutable Array
  let mut a: [i32; 6] = [1, 2, 3, 4, 5, 6];
  a[1] = 1;

  if a == [1, 1, 3, 4, 5, 6] {
    println!("Match!");
  }
  // Constant Repeating Array
  let b: [u8; 10] = [6u8; 10];

  // for (i, i2) in (5..b.len()).enumerate() {
  //   println!("{}: {}", b[i], i2);
  // }

  // Two-Dimensional Repeating Array
  let mtx: [[f32; 3]; 2] = [[0.0; 3], [1.0; 3]];

  for i in 0..mtx.len() {
    for j in 0..mtx[i].len() {
      if i != j {
        continue;
      }
      println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
    }
  }
}
