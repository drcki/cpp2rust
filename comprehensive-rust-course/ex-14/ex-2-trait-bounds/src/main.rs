fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

#[derive(Clone, Debug)]
struct NotCloneable;

fn main() {
  let foo = String::from("foo");
  let pair = duplicate(foo);
  println!("{pair:?}");


  let nc = NotCloneable;
  let pair = duplicate(nc);
  println!("{pair:?}");
}


// Note that Rust does not (yet) support specialization.