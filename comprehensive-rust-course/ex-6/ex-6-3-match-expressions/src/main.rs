/*
https://google.github.io/comprehensive-rust/control-flow-basics/match.html
*/

fn main() {
    let val = 2;

    // as some sort of switch
    match val {
      1 => println!("one"),
      2 => println!("two"),
      100 => println!("one hundred"),
      _ => {
        println!("something else");
      }
    }


    // returning a value
    let flag = true;
    let val = match flag {
      true => 1,
      false => 0,
    };
    println!("The value of {flag} is {val}");
}
