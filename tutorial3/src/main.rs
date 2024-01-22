// you can give custom instructions to teh compiler with the following line
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // immutable variable by default
    let x: i32 = 4;
    println!("The value of x is: {}", x);

    let mut y: i32 = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is now: {}", y);

    // you can also re-define variables
    let i = 1;
    let i = i + 1;
    println!("The value of i is: {}", i);

    show_scopes();

    use_constants();
}

fn show_scopes() {
    println!("showScopes()");

    let x: i32 = 17;
    // this is a new scope
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // println!("The value of x is {} and value of y is {}", x, y); // this will fail
    let x = x + 3;
    // now x is 20
    println!("The value of x is now {}", x);
}

fn use_constants() {
    println!("useConstants()");

    // you have to specify the type of the constant
    const MAX_POINTS: u32 = 100_000;

    // you cannot redefine a constant

    println!("The value of MAX_POINTS is {}", MAX_POINTS);
}
