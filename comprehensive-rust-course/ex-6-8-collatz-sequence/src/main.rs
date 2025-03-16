/*
    https://google.github.io/comprehensive-rust/control-flow-basics/exercise.html
*/

fn collatz_length(mut n: i32) -> u32 {
  let mut length = 1;
  while n != 1 {
    length += 1;
    if n % 2 == 0 {
      n /= 2;
      continue;
    }
    n = n*3+1;
  }
  length
}


fn main() {
  let length = collatz_length(11);
  println!("length: {length}");
  assert_eq!(length, 15);
}