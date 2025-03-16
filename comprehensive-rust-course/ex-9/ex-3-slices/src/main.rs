// https://google.github.io/comprehensive-rust/references/slices.html

fn main() {
    let a = [10,20,30,40,50,60];
    println!("a: {a:?}");

    let s = &a[2..4]; // slice
    println!("s: {s:?}");

    let z  = &a[..2];
    println!("z: {z:?}");

    // Does z become freed after last usage ??? 

    let z  = &a[3..];
    println!("z: {z:?}");

    let z  = &a[..];
    println!("z: {z:?}");
}

