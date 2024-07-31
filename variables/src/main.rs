fn types() {
    // integers
    let _a: i32 = 98_000;
    let _b: i32 = 0xff;
    let _c: i32 = 0o77;
    let _d: i32 = 0b1111_0000;
    let _e: u8 = b'A';

    let _f: u8 = 255;

    // floating-point numbers

    let _f = 2.0;
    let _g: f32 = 3.0;

    // addition
    let _sum = 5 + 10;
    // sub
    let _difference = 95.5 - 4.3;
    // division
    let _quotient = 56.7 / 32.2;
    // remainder
    let _remainder = 43 % 5;

    // booleans

    let _t = true;

    let _f = false;

    // character

    let _c = "z";
    let _z = "Z";
    let _heart_eyed_cat = "🐱";
}

fn compond_types() {
    // tuple
    let tup: (&str, i32) = ("Let's Get Rusty", 100_000);

    let (_channel, _sub_count) = tup; // destructuring
    let _sub_count = tup.1; // dot notation

    // array
    let error_codes = [200, 404, 500]; // basic array
    let _not_found = error_codes[1];
    // let _x = error_codes[3]; // we get error

    let _byte = [0; 8]; // create 8 elements in array all set to 0 (zero) [0,0,0,0,0,0,0,0]
}

// functions
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
    let sum = x + y;
    sum
}

fn control_flow() {
    let number = 32;
    if number > 15 {
        println!("1-condition was true");
    } else if number < 22 {
        println!("2-condition was true");
    } else {
        println!("condition was false");
    }
}

fn main() {
    let x: isize = 58000;
    println!("The value of x is: {}", x);
    let x = "six"; // shadowing
    println!("The value of x is: {}", x);

    #[allow(dead_code)]
    const SUBSCRIBER_COUNT: u32 = 10000;

    types();
    compond_types();
    let _sum = my_function(12, 43);
    control_flow();
}
