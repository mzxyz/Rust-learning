
// 1. Checking Results with the assert! Macro

/// The assert! macro, provided by the standard library, 
/// is useful when you want to ensure that some condition 
/// in a test evaluates to true. We give the assert! 
/// macro an argument that evaluates to a Boolean. If the 
/// value is true, assert! does nothing and the test passes. 
/// If the value is false, the assert! macro calls the panic! 
/// macro, which causes the test to fail. Using the assert! 
/// macro helps us check that our code is functioning in the way we intend.

// 2. Testing Equality with the assert_eq! and assert_ne! Macros

/// Under the surface, the assert_eq! and assert_ne! macros use the 
/// operators == and !=, respectively. When the assertions fail, 
/// these macros print their arguments using debug formatting, which 
/// means the values being compared must implement the PartialEq 
/// and Debug traits. All the primitive types and most of the standard 
/// library types implement these traits. For structs and enums that 
/// you define, you’ll need to implement PartialEq to assert that values 
/// of those types are equal or not equal. You’ll need to implement Debug 
/// to print the values when the assertion fails. Because both traits 
/// are derivable traits, this is usually as straightforward as 
/// adding the #[derive(PartialEq, Debug)] annotation to your struct or 
/// enum definition.

// 3. Adding Custom Failure Messages
/// Custom messages are useful to document what an assertion means; 
/// when a test fails, you’ll have a better idea of what the problem 
/// is with the code.
///  Any arguments specified after the one required argument to assert! 
/// or the two required arguments to assert_eq! and assert_ne! are passed
/// along to the format! macro 

pub fn greeting(name: &str) -> String {
    String::from(name)
}

// 4. Checking for Panics with should_panic
/// In addition to checking that our code returns the 
/// correct values we expect, it’s also important to 
/// check that our code handles error conditions as we expect.
pub struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }

        Guess {
            value
        }
    }
}

// 5. Using Result<T, E> in Tests
/// The it_works function now has a return type, Result<(), String>. 
/// In the body of the function, rather than calling the assert_eq! 
/// macro, we return Ok(()) when the test passes and an Err with a 
/// String inside when the test fails.
/// You can’t use the #[should_panic] annotation on tests that use 
/// Result<T, E>. Instead, you should return an Err value directly 
/// when the test should fail.

mod control_test_run;
mod test_organization;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greeting_not_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Hello"),
            "Greeting did not contain `Hello`, value was {}", result
        );
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain `Hello`, value was {}", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // note: Panic should include expected string 'Guess value must be less than or to 100'
    fn guess_greater_than_100() {
        let _g = Guess::new(200);
    }

    #[test]
    // running single test: cargo test it_works
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 plus 2 not equal 4"))
        }
    }

    #[test]
    #[ignore]
    fn test_ignore() {
        // run `cargo test -- --ignored`
        assert!(2 == 1 + 1);
    }
}
