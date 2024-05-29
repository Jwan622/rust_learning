fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // addition
    let sum = 5 + 10;
    println!("The value of sum is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    // booleans are 1 byte
    let t = true;
    let f: bool = false; // with explicit type annotation

    // tuples and destructuring
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is {x}");
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}