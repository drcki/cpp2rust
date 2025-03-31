// https://google.github.io/comprehensive-rust/user-defined-types/enums.html

/*
Niches are ranges of invalid
(i.e., unused) values in a type’s representation that can be used to store other data. Booleans are
an example of a type with a niche; on many CPUs, the smallest unit of memory is a byte, while
booleans only use the values 0 and 1. As a result, the bool type possesses a niche in the range
[2, 255].
*/

fn main() {
    println!("Hello, world!");
}
