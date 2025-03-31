fn gcd(a: u32, b: u32) -> u32 {
  if b > 0 {
      gcd(b, a % b) // missing ; - return value
  } else {
      a
  }
}

fn main() {
  dbg!(gcd(143, 52));
}