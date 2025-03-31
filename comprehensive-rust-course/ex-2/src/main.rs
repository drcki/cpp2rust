fn main() {
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");

    let floating: f32 = -10.0e2;
    println!("floating: {floating}");

    let another_floating: f32 = 2.1_f32;
    println!("another_floating: {another_floating}");

    let yet_same_floating: f32 = 2.1f32;
    println!("yet_same_floating: {yet_same_floating}");

    // println!("result: {}", interproduct(120, 100, 248));

    println!("interproduct_2: {}", interproduct_2(120, 100, 248))
}

// fn interproduct(a: i32, b: i32, c:i32) -> i16 {
//   // return a * b + b * c + c * a;
//   return (a * b + b * c + c * a).try_into().unwrap();
// }

fn interproduct_2(a: i32, b: i32, c: i32) -> i16 {
    return (a * b)
        .saturating_add(b * c)
        .saturating_add(c * a)
        .try_into()
        .unwrap();
}
