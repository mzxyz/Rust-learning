// 1. Function Pointers
/// Unlike closures, fn is a type rather than a trait, 
/// so we specify fn as the parameter type directly rather 
/// than declaring a generic type parameter with one of 
/// the Fn traits as a trait bound.
pub fn add_one(x: i32) -> i32 {
  x + 1
}

pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}
 
/// As an example of where you could use either a closure defined inline 
pub fn list_of_strings_closure(strs: &Vec<i32>) -> Vec<String> {
  strs.iter()
    .map(|i| i.to_string())
    .collect()
}

/// Name a function as the argument to map instead of the closure
/// Must use the fully qualified syntax, as there are multiple functions 
/// available named to_string. Here, we’re using the to_string function 
/// defined in the ToString trait, which the standard library has 
/// implemented for any type that implements Display.
pub fn list_of_strings_fn(strs: &Vec<i32>) -> Vec<String> {
  strs.iter()
    .map(ToString::to_string)
    .collect()
}

/// The initializers are actually implemented as functions returning
/// an instance that’s constructed from their arguments
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Status {
  Value(u32),
  Stop
}

/// We can use these initializer functions as function pointers 
/// that implement the closure traits, which means we can specify 
/// the initializer functions as arguments for methods that take closures
pub fn list_of_statuses(t: Range<u32>) -> Vec<Status> {
  t.map(Status::Value).collect()
}

// 2. Returning Closures
/// Closures are represented by traits, which means you can’t return 
/// closures directly. In most cases where you might want to return 
/// a trait, you can instead use the concrete type that implements 
/// the trait as the return value of the function.

/// But you can’t do that with closures because they don’t have a 
/// concrete type that is returnable; you’re not allowed to use 
/// the function pointer fn as a return type, for example.

// *******************************************************************************//
// The following code tries to return a closure directly, but it won’t compile
// *******************************************************************************//
// fn return_closure() -> Fn(i32) -> i32 {
//   |x| x + 1
// }
// 
// error[E0277]: the trait bound `std::ops::Fn(i32) -> i32 + 'static: 
// std::marker::Sized` is not satisfied
// ust doesn’t know how much space it will need to store the closure.
pub fn return_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fn_pointer_as_parameter() {
    let a = do_twice(add_one, 2);
    assert_eq!(a, 6);
  }

  #[test]
  fn using_closure_or_named_fn() {
    let list_of_numbers = &vec![1, 2, 3];
    let list_of_strings = vec!["1", "2", "3"];

    assert_eq!(list_of_strings_closure(list_of_numbers), list_of_strings);
    assert_eq!(list_of_strings_fn(list_of_numbers), list_of_strings);
  }

  #[test]
  fn test_list_of_statuses() {
    let t = 0u32..20;
    let v: Vec<Status> = (0u32..20).map(|i| Status::Value(i)).collect();
    assert_eq!(list_of_statuses(t), v);
  }

  #[test]
  fn test_return_closure() {
    let c = return_closure();
    assert_eq!(c(1), 2);
  }
}