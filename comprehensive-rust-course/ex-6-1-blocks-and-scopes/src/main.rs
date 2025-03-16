

/*
https://google.github.io/comprehensive-rust/control-flow-basics/blocks-and-scopes.html
 */
fn main() {
  let z = 13;
  let x = {
    let y = 10;
    dbg!(y);
    z - y
    // let _o= z - y;
  };
  dbg!(x);
  // dbg!(y); // error[E0425]: cannot find value `y` in this scope
}
