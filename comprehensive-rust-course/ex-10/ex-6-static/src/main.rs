// https://google.github.io/comprehensive-rust/user-defined-types/static.html

static BANNER: &str = "Welcome to RustOS 3.14";

/*
static provides object identity: an address in memory and state as required by types with interior mutability such as Mutex<T>.
*/

fn main() {
    println!("{BANNER}");
}