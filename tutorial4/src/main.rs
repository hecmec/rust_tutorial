fn main() {
    println!("Hello, world!");

    let o: i8 = 1;
    let p: i16 = 2;
    let x: i32 = 5;
    let y: i64 = 6;
    let z: i128 = 7;
    let x2: u8 = 88; // u8 is unsigned 8-bit integer 0 .. 255
    let y2: i8 = 99; // i8 is signed 8-bit integer -128 .. 127

    // float
    let a: f32 = 1.0; // Regular annotation, single precision
    let b: f64 = 2.0; // Regular annotation, double precision, this is the default type

    // boolean
    let is_good: bool = true;
    let is_bad: bool = false;
    println!("is_good: {}", is_good);
    println!("is_bad: {}", is_bad);
    // let is_good2: bool = 1;
    // let is_bad2: bool = 0;

    // chars
    let c: char = 'z'; // only one character, char is 4 bytes in size and represents a Unicode Scalar Value
    println!("c: {}", c);

    // tuples
    let tup: (i32, f64, bool, char) = (500, 6.4, true, 'a');
    println!("tup: {:?}", tup);
    println!("tup.0: {}", tup.0);
    println!("tup.1: {}", tup.1);
    println!("tup.2: {}", tup.2);

    // you can make this tuple mutable
    let mut tup2: (i32, bool, char) = (1, false, 's');
    tup2.0 = 2;
    println!("tup2: {:?}", tup2);

    // arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    println!("arr[0]: {}", arr[0]);
}
