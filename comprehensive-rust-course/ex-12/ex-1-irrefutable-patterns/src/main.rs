// https://google.github.io/comprehensive-rust/pattern-matching/infallible.html

/*
All of the demonstrated patterns are irrefutable, meaning that they will always match the value on the right hand side.
*/

fn takes_tuple(tuple: (char, i32, bool)) {
  let a = tuple.0;
  let b = tuple.1;
  let c = tuple.2;

  let (a, b, c) = tuple;

  let (_, b, c) = tuple;

  let (.., c) = tuple;
}




fn main() {
    takes_tuple(('a', 777, true));
}
