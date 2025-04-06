struct Foo {
  x: (u32, u32),
  y: u32,
  z: u32
}

#[rustfmt::skip]
fn main() {
  let foo = Foo { x: (2, 2), y: 2, z: 10 };
  let two = 2u32;
  match foo {
    Foo { x: (1, b), y , z:_} => println!("x.0 = 1, b = {b}, y = {y}"),
    Foo { y: two, x: i, ..}   => println!("y = 2, x = {i:?}"),
    Foo { y,  z,..}        => println!("y = {y}, z = {z}, other fields were ignored"),
  }


  match &foo {
    Foo { x: (1, b), y , z:_} => println!("x.0 = 1, b = {b}, y = {y}"),
    Foo { y: 2, x: i, ..}   => println!("y = 2, x = {i:?}"),
    Foo { y,  z,..}        => println!("y = {y}, z = {z}, other fields were ignored"),
  }
}