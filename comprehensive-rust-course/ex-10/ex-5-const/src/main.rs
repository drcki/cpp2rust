// https://google.github.io/comprehensive-rust/user-defined-types/const.html

/*
Similar to constexpr
*/

const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 {
        42
    } else {
        13
    }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest: [u8; 3]  = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest: [u8; 3] = compute_digest("Hello");

    println!("digest: {digest:?}");
}
