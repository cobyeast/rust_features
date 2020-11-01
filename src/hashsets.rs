use std::collections::HashSet;

fn hashset() {
  let mut greek = HashSet::new();

  greek.insert("gamma");
  greek.insert("delta");

  if let Some(add_vega) = greek.get("vega") {
    print!("{:?}\n", add_vega);
  }

  if !greek.contains("kappa") {
    greek.insert("kappa");
  }

  greek.remove("delta");

  println!("{:?}", greek);

  let one_5: HashSet<i32> = (1..=5).collect();
  let two_8: HashSet<i32> = (2..=8).collect();
  let six_10: HashSet<i32> = (6..=10).collect();
  let one_10: HashSet<i32> = (1..=10).collect();

  // Subset
  print!("is subset = {:?}\n", six_10.is_subset(&one_10));

  // Disjoint
  print!("is disjoint = {:?}\n", one_5.is_disjoint(&six_10));

  // Union
  // Returns a set of common numbers or the numbers contained in each set
  print!("is union = {:?}\n", one_5.union(&two_8));

  // Symmetric difference
  // Returns a set of uncommon numbers or the numbers unique to each set
  print!(
    "is symmetric_difference = {:?}\n",
    one_5.symmetric_difference(&two_8)
  );
}

pub fn hashsets() {
  hashset()
}
