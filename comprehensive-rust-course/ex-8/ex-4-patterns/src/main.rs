fn main() {
  // let tuple = (1, 5, 3);
  let tuple = (1, 2, 3);
    println!("{tuple:?}: {}", if check_order(tuple) {"ordered"} else {"unordered"});
}

fn check_order(tuple: (i32, i32, i32)) -> bool {
  let (left, middle, right) = tuple;
  left < middle && middle < right
}