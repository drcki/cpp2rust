fn main() {
  let mut i = 0;
  loop {
    i += 1;
    if i > 5 {
      break;
    }
    if i % 2 == 0 {
      continue;
    }
    dbg!(i);
  }



  /*
      Labels
  */
  let s = [[5,6,7],[8,9,10],[11,12,13]];
  let mut elements_serched = 0;
  let target_value = 10;

  'outer: for i in 0..=2 {
    for j in 0..=2 {
      elements_serched += 1;
      if s[i][j] == target_value {
        break 'outer;
      }
    }
  }
  dbg!(elements_serched);



  // arbitrary blocks
  'label: {
    break 'label;
    println!("This line gets skipped");
  }

}
