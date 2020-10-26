fn operators() {
  let mut a: f32 = 2.5 + 3.4 * 4.2;
  a += 1.5;

  let a_cubed = f32::powi(a, 3);
  println!("a = {}, a_cubed = {}", a, a_cubed);

  let c = (1 == 10) == false;

  println!("c = {}", c)
}
