fn main() {
    let max_value = 10;

    let clamp = move |v| {
        if v > max_value {
            max_value
        } else {
            v
        }
    };

    dbg!(clamp(1));
    dbg!(clamp(5));
    dbg!(clamp(10));
    dbg!(clamp(15));

    dbg!(max_value);
    
    dbg!(clamp(1));
    dbg!(clamp(5));
    dbg!(clamp(10));
    dbg!(clamp(15));
}
