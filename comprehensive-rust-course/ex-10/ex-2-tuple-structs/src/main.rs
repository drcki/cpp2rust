// https://google.github.io/comprehensive-rust/user-defined-types/tuple-structs.html

// 1st ex

// struct Point(i32, i32);

// fn main() {
//     let p: Point = Point(17, 23);
//     println!("Point: ({}, {})", p.0, p.1);
// }



// 2nd ex

// struct PoundsOfForce(f64);
// struct Newtons(f64);

// fn compute_thruster_force() -> PoundsOfForce {
//     todo!("Not ready yet")
// }

// fn set_thruster_force(_force: Newtons) {
//     todo!("Not ready yet")
// }

fn main() {
    // let _force = compute_thruster_force();

    // set_thruster_force(force); // type mismatch
}