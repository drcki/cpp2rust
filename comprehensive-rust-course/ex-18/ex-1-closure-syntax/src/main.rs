fn main() {
    let double_it = |n| n * 2;
    dbg!(double_it(50));

    let add_if32 = |x: f32| -> f32 { x + 1.0 };
    dbg!(add_if32(50.));
}
