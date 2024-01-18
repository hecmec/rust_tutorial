// crate std is the standard library
// io is the input/output library
use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();

    println!("Please input something: ");

    // &mut input is a reference to the variable input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You typed: {}", input);

    // let result = io::stdin().read_line(&mut input);
    // println!("result: {:?}", result.unwrap());
}
