use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// define the Cacher struct that holds a closure and an optional result value.
struct Cacher<T> where
    T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>
}

impl<T> Cacher<T> where
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => !v,
            None => {
                // to call the function stored in `calculation`, surround the field access with parentheses
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today");
        } else {
            println!("Today, run for {} minutes",
            expensive_closure.value(intensity))
        }
    }
}

// closures can capture their environment and access variables
// from the scope in which they’re defined. But function can not do this
fn capture_var() {
    let x = 2;

    // When a closure captures a value from its environment, it uses 
    // memory to store the values for use in the closure body. This use 
    // of memory is overhead that we don’t want to pay in more common 
    // cases where we want to execute code that doesn’t capture its environment
    let equal_to_x = |a| a + x;

    // can use `move` to force the closure to take ownership of the values it uses in the environment
    // let equal_to_x = move |z| z == x;

    // Closures can capture values from their environment in three ways, which 
    // directly map to the three ways a function can take a parameter: taking 
    // ownership, borrowing mutably, and borrowing immutably

    // this will not work. Can't capture dynamic environment in a fn item
    // fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;
    assert_eq!(y, equal_to_x(2));
}



fn main() {
    generate_workout(20, 5);
    generate_workout(26, 3);
    generate_workout(28, 1);

    capture_var();
}

// fn  add_one(x: u32) -> u32 { x + 1 }
// let add_one = | x: u32 | -> u32 { x + 1 };
// let add_one = |x| x + 1;


// test
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    let v3 = c.value(344);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 344);
}