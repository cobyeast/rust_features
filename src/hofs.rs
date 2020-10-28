fn is_even(x: i32) -> bool {
  if x % 2 == 0 {
    true
  } else {
    false
  }
}

// implement the Fn trait
fn greater_than(limit: i32) -> impl Fn(i32) -> bool {
  // move is needed for lifetime extension
  move |y| y > limit
}

pub fn hofs() {
  let limit = 500;
  let mut sum = 0;

  let above_limit = greater_than(limit);

  for i in 0.. {
    let isq = i * i;
    if above_limit(isq) {
      break;
    } else if is_even(isq) {
      sum += isq;
    }
  }

  println!("sum = {}", sum);

  let sum2 = (0..)
    // map() takes lambda with value and tranforms to another value
    // @params (|x|) is a copied value
    .map(|x| x * x)
    // take_while() takes lambda with sequence elements while condition is true
    // @params (|&x|) x is a reference
    .take_while(|&x| x < limit)
    // filter() takes lambda with value
    // @params (|x: &i32|) x is a reference
    .filter(|x: &i32| is_even(*x))
    // fold() folds sequence into single value through pairwise operations
    // @params (0, fn(|sum, x|)) 0 is the initial value, fn is a function with lambda arg
    // @params fn has a lambda with two args (|sum, x|) sum is the total at point in time, x is value at each iteration
    .fold(0, |sum, x| sum + x);

  println!("sum2 = {}", sum2);
}
