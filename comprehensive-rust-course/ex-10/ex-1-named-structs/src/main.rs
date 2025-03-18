struct Person {
  name: String,
  age: u8,
}

fn describe(person: &Person) {
  println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let peter: &mut Person = &mut Person {
      name: String::from("Peter"),
      age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name: String = String::from("Every");
    let age: u8 = 39;
    let avery: Person = Person { name, age };
    describe(&avery);

    // let p: Person = Person { String::from("Peter"), 12}; // can't do like that,
    // have to define like designated initializer or variable (with matching name!)


    // struct update
    let jackie = Person { name: String::from("Jackie"), ..avery};
    describe(&jackie);

    
}


/*
    struct Test {
      a: i32,
      b: i32,
      c: i32,
    }

    let testo: &mut Test = &mut Test{ a: 1, b:9, c:3 };
    describe_test(&testo);
    // 193
    let testo2: Test = Test{ a: 9, c: 8, ..*testo };
    describe_test(&testo2);
    // 998

    .. - fills all data not mentioned in assignemt before

*/

