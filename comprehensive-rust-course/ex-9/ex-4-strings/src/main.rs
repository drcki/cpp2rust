fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello");
    println!("s2: {s2}" );

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3}");

    
    println!("{:?}", "abc");
    println!("{:?}", b"abc");
    println!("{:?}", &[97,98,99]);
    
    println!(r#"\\\\<a href="link.html">link</a>"#); // r - escapes disabled
    println!("\\\\<a href=\"link.html\">link</a>");
}
