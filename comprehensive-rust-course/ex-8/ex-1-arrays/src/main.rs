fn main() {
    // let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    let mut a: [i8; 5] = [1; 5];
    a[2] = 0;

    println!("a: {a:?}");
    println!("a: {a:#?}");

    // println!("out of bounds a[5]: {}", a[5]); // index out of bounds: the length is 5 but the index is 5
}
