/// varibles
fn immutable_test() {
    let x = 5;
    // println! is a macro
    println!("The value of x is: {}", x);

    // can not assign twice to immutable varible x
    // x = 4;
    // println!("The new value of x is {}", x);
}

fn mutable_test() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value of x is: {}", x);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The shadow value of x is: {}", x);
}

/// types
/// scalar types
/// integer tyeps
/// i8 (-128 -> 127) / u8 (0 -> 255) | i16 / u16 ... | i128 / u128
// isize / usize (64-bit or 32 bits architecture computer)
fn float_point() {
    let _x = 2.0; // f64 by default
    let _y: f32 = 3.0;
}

/// addition, subtraction, multiplication, division, and remainder
fn number_operations() {
    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    // remainder
    let _remainder = 43 % 5;
}

/// boolean
fn boolean_type() {
    let _t = true;
    let _f: bool = false;
}

/// char
fn char_type() {
    // Rust’s char type represents a Unicode Scalar Value
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    let _c = 'z';
}

/// tuples have a fixed length: once declared, they cannot grow or shrink in size.
/// values with a variety of types 
fn tuple_types() {
    let t: (i64, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = t;
    let x = t.0;
    println!("The value of x in tuple: {}", x);
}

/// array has fixed length
/// the elements can be modified, it cannot grow or shrink
fn array_type() {
    // [type; number]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _e = a[0];
}

/// !!!functions
/// statements and expressions
/// statement doesn't return a value
/// Expressions do not include ending semicolons. (fn, if...) 
/// If you add a semicolon to the end of an expression, you turn it into a statement, 
/// which will then not return a value. Keep this in mind as you explore function return values and expressions next.
fn plus_one(x: i32) -> i32 {
    x + 1
}

/// control fow
fn if_example(condition: bool) {
    let x = 3;
    // Rust will not automatically try to convert non-Boolean types to a Boolean
    if x != 2 {
        println!("x is not equal to 2");
    }

    let _n = if condition {
        5
    } else {
        // all types in if flow should be same, can not be string or other types
        // Rust wouldn’t be able to do that if the type of number was only determined at runtime;
        6
    };
}

/// loop
fn loop_ex() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

/// while
/// for
fn for_ex() {
    let a = [10, 11, 12, 12];
    for i in a.iter() {
        println!("the value in array is {}", i);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF");
}

fn main() {
    immutable_test();
    mutable_test();
    shadowing();

    float_point();
    number_operations();
    boolean_type();
    char_type();
    tuple_types();
    array_type();
    plus_one(3);

    if_example(true);
    loop_ex();
    for_ex();
}