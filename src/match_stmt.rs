pub fn match_statement() {
  let country: u8 = 100;

  let code = match country {
    44 => "UK",
    20 => "US",
    (1..=100) => "unknown",
    _ => "invalid",
  };

  println!("{}", code);
}
