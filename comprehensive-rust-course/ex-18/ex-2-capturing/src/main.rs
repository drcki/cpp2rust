fn main() {
    let mut max_value = 10;

    let mut clamp = move |v| {
        if v > max_value {
            // max_value += 1;
            max_value
        } else {
            v
        }
    };

    dbg!(clamp(5));
    dbg!(clamp(15));

    // dbg!(max_value);

    // max_value = 20; // use of borrowed value: `max_value`
    dbg!(max_value);
    
    dbg!(clamp(5));
    dbg!(clamp(25)); // max_value is still 10+2
}
