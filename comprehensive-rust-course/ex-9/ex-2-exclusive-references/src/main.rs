/*
    https://google.github.io/comprehensive-rust/references/exclusive.html
*/

fn main() {
    let mut point = (1,2);
    let x_coord = &mut point.0;

    // let rr = &point.0; // error[E0502]: cannot borrow `point.0` as immutable because it is also borrowed as mutable
    // dbg!(*rr);

    *x_coord = 20;

    
    let rr = &point.0; // when x_coord is not alive? Compiler allows such operation what is not allowed above.
    dbg!(*rr);

    // x_coord = &mut point.1; // error[E0384]: cannot assign twice to immutable variable `x_coord`

    println!("point: {point:?}");
}
