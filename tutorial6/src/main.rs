fn main() {
    let x: u8 = 18;
    let y: i8 = -9;

    let o = 255_f32;
    let p = 10_f32;
    println!("o: {}, p: {}, o/p: {}", o, p, o / p);

    // you can only add variables of the same type
    // as to do explicit conversion
    let z = x as i8 + y;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let a: u8 = 255;
    let b: u8 = 10;
    let c = a / b; // c is 25 here
    println!("a: {}, b: {}, c: {}", a, b, c);

    println!("25%10: {}", 25 % 10);
}
