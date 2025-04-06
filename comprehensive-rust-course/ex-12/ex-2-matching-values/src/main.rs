#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"),
    }


    let opt = Some(123);
    match opt {
      outer @ Some(inner) => {
          println!("outer: {outer:?}, inner: {inner}");
      }
      None => {}
    }

}