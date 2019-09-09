// The Rust community thinks about tests in terms of 
// two main categories: unit tests and integration 
// tests. Unit tests are small and more focused, 
// testing one module in isolation at a time, and 
// can test private interfaces. Integration tests 
// are entirely external to your library and use 
// your code in the same way any other external code
// would, using only the public interface and potentially 
// exercising multiple modules per test.

// 1. Unit Tests
/// The #[cfg(test)] annotation on the tests module 
/// tells Rust to compile and run the test code only 
/// when you run cargo test, not when you run cargo build.
/// This saves compile time when you only want to build the 
/// library and saves space in the resulting compiled artifact 
/// because the tests are not included
/// However, because unit tests go in the same files as the code, 
/// you’ll use #[cfg(test)] to specify that they shouldn’t be included 
/// in the compiled result.

// 2. Integration Tests
/// In Rust, integration tests are entirely external to your library.
/// Cargo treats the tests directory specially and compiles files in 
/// this directory only when we run cargo test

/// We can still run a particular integration test function by specifying 
/// the test function’s name as an argument to cargo test. To run all the 
/// tests in a particular integration test file, use the --test argument 
/// of cargo test followed by the name of the file:
/// `cargo test --test integration_test`