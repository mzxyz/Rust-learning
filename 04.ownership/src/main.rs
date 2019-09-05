/// Ownership is Rust’s most unique feature,
/// and it enables Rust to make memory safety 
/// guarantees without needing a garbage collector. 


/// ownership rules
/// 1. Each value in Rust has a variable that’s called its owner.
/// 2. There can only be one owner at a time.
/// 3. When the owner goes out of scope, the value will be dropped.

fn string_type() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn move_var() {
    let s1 = String::from("Hello");
    let _s2 = s1; // move s1 to s2, the reference of s1 will be moved
    // it is now shallow copy, it's move
}

fn deepcopy() {
    let s1 = String::from("Hello");
    let _s2 = s1.clone(); // the heap data get copied
}

fn calculate_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

/// The &s1 syntax lets us create a reference that 
/// refers to the value of s1 but does not own it.
/// Because it does not own it, the value it points 
/// to will not be dropped when the reference goes out of scope.
fn reference(s: &String) -> usize {
    s.len()
}

/// At any given time, you can have either (but not both of) one 
/// mutable reference or any number of immutable references.
/// References must always be valid.
fn mut_reference(s: &mut String) {
    s.push_str(", world");
}

/// get first word -> slice
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}

fn slice() {
    let s = String::from("Hello World");
    let h = &s[0..=4];
    let w = &s[6..];
    let t = &s[..];
    println!("{} {} {}", h, w, t);
}

fn main() {
    string_type();
    move_var();
    deepcopy();

    // reference
    let s1 = String::from("Hello");
    let (s2, _len) = calculate_len(s1);
    reference(&s2);

    // mut reference
    let mut s3 = String::from("Hello");

    // let r2 = &mut s3; this will throw error
    // you can only have one mutable reference to 
    // a particular piece of data in a particular scope.
    mut_reference(&mut s3);

    // right code
    {
        let _r2 = &mut s3;
    }


    // slice
    slice();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let s = String::from("Hello World");
        let w = first_word(&s);
        assert_eq!(w, "Hello".to_string());
    }
}
