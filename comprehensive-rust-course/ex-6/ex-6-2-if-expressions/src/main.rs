fn main() {
    let x = 10;
    if x == 0 {
      println!("zero!");
    } else if x < 100 {
      println!("biggish");
    } else {
      println!("huge");
    }

    let z = 10;
    // let size = if z < 20 {"small"}; // else {"large"}; // error[E0317]: `if` may be missing an `else` clause
    let size = if z < 20 {"small"} else {"large"};
    
    println!("number size: {}", size);
}
