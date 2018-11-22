fn main() {
    immutable_test();
    mutable_test();
    shadowing();
}

fn immutable_test() {
    let x = 5;
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