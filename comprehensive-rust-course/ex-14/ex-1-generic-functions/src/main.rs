// https://google.github.io/comprehensive-rust/generics/generic-functions.html

fn pick<T>(cond: bool, left: T, right: T) -> T {
  if cond {
      left
  } else {
      right
  }
}

// Rust infers a type for T based on the types of the arguments and return value.


/*
This is similar to C++ templates, but Rust partially compiles the generic function immediately,
so that function must be valid for all types matching the constraints.

For example, try modifying pick to return even + odd if n == 0.
Even if only the pick instantiation with integers is used, Rust still considers it invalid.
C++ would let you do this.
*/

// fn sum<T>(n: u32, even: T, odd: T) -> T {
//   even + odd
// }

fn main() {
  println!("picked a number: {:?}", pick(true, 222, 333));
  println!("picked a string: {:?}", pick(false, 'L', 'R'));

  // println!("sum: {:?}", sum(2, 2, 1));
}