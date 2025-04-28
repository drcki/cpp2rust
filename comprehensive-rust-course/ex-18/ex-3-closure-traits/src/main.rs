fn apply_and_log(func: impl FnOnce(String) -> String, func_name: &str, input: &str) {
    println!("Calling {func_name}({input}): {}", func(input.to_string()))
}

fn main() {
    let suffix = "-itis";
    let add_suffix = |x| format!("{x}{suffix}");
    apply_and_log(&add_suffix, "add_suffix", "senior");
    apply_and_log(&add_suffix, "add_suffix", "appenix");

    let mut v = Vec::new();
    let mut accumulate = |x| {
        v.push(x);
        v.join("/")
    };
    apply_and_log(&mut accumulate, "accumulate", "red");
    apply_and_log(&mut accumulate, "accumulate", "green");
    apply_and_log(&mut accumulate, "accumulate", "blue");

    let take_and_reverse = |mut prefix: String| {
        prefix.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
        prefix
    };
    apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");
}