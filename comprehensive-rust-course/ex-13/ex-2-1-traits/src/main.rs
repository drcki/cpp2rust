// https://google.github.io/comprehensive-rust/methods-and-traits/traits/implementing.html

trait Pet {
  fn talk(&self) -> String;

  fn greet(&self) {
      println!("Oh you're a cutie! What's your name? {}", self.talk());
  }
}

struct Dog {
  name: String,
  age: i8,
}

impl Pet for Dog {
  fn talk(&self) -> String {
      format!("Woof, my name is {}!", self.name)
  }
}

fn main() {
  let fido = Dog { name: String::from("Fido"), age: 5 };
  fido.greet();
}