#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

// example
impl From<&str> for Foo {
    fn from(from: &str) -> Foo {
      Foo(format!("Converted from str: {from}"))
    }
}

fn main() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    dbg!(from_int);
    dbg!(from_bool);


    /*
    Implementations of the trait do not need to cover all possible type parameters.
    Here, Foo::from("hello") would not compile because there is no From<&str> implementation for Foo.
    */
    let from_str = Foo::from("Hello");
    dbg!(from_str);
}

