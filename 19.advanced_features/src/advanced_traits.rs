// 1. Specifying Placeholder Types in Trait Definitions with Associated Types
pub trait Iterator {
  type Item;
  fn next(self) -> Option<Self::Item>;
}

#[allow(dead_code)]
struct Counter<T>(T);

// With associated types, we don’t need to annotate types like using generics
// because we can’t implement a trait on a type multiple times. 
impl <T>Iterator for Counter<T> {
  type Item = Self;

  fn next(self) -> Option<Self::Item> {
    Some(self)
  }
}

// 2. `Default Generic Type Parameters` and `Operator Overloading`
// The syntax for specifying a default type for a generic type 
// is <PlaceholderType=ConcreteType> when declaring the generic type.
//  Operator overloading is customizing the behavior of an operator (such as +) in particular situations.
/// Rust doesn’t allow you to create your own operators or overload arbitrary 
/// operators. But you can overload the operations and corresponding traits listed in std::ops 
/// by implementing the traits associated with the operator
use std::ops::Add;

#[derive(Debug, PartialEq)]
  pub struct Point<T> {
    pub x: T,
    pub y: T
  }

impl <T: Add<Output = T>> Add for Point<T> {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Point {
      x: self.x + other.x,
      y: self.y + other.y
    }
  }
}

/// The default generic type in this code is within the Add trait. Here is its definition:
/// The new part is `RHS=Self:` this syntax is called default type parameters. The RHS generic 
/// type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method. 
/// 
// trait Add<RHS=Self> {
//   type Output;

//   fn add(self, rhs: RHS) -> Self::Output;
// }
#[derive(Debug, PartialEq)]
pub struct Millimeters(pub u32);

#[derive(Debug)]
pub struct Meters(pub u32);

// You’ll use default type parameters in two main ways:
// - To extend a type without breaking existing code
// - To allow customization in specific cases most users won’t need

/// if you want to add a `type parameter` to an existing trait, you 
/// can give it a default to allow extension of the functionality 
/// of the trait without breaking the existing implementation code.
impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + other.0 * 1000)
  }
}

// 3. Using operator traits in generic structs
use std::ops::Mul;

pub trait HasArea<T> {
  fn area(&self) -> T;
}

pub struct Square<T> {
  pub x: T,
  pub y: T,
  pub side: T
}

impl<T> HasArea<T> for Square<T> where
  T: Mul<Output=T> + Copy
{
  fn area(&self) -> T {
    self.side * self.side
  } 
}

// 4. Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
/// associated functions that are part of traits don’t have a self parameter. When 
/// two types in the same scope implement that trait, Rust can’t figure out which 
/// type you mean unless you use fully qualified syntax. 

// Because Animal::baby_name is an associated function rather than a method, and 
// thus doesn’t have a self parameter, Rust can’t figure out which implementation 
// of Animal::baby_name we want. For normal trait method, we could use:
// `Animal::baby_name(&dog)` to use `baby_name`

// We’re providing Rust with a `type annotation` within the angle brackets, which 
// indicates we want to call the baby_name method from the Animal `trait as`
// implemented on Dog by saying that we want to `treat the Dog type as an Animal` for this function call
///!! <Type as Trait>::function(receiver_if_method, next_arg, ...);
/// You only need to use this more verbose syntax in cases where there are multiple implementations 
/// that use the same name and Rust needs help to identify which implementation you want to call.
pub trait Animal {
  fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("Puppy")
  }
}

// 5. Using Supertraits to Require One Trait’s Functionality Within Another Trait
/// The trait you rely on is a supertrait of the trait you’re implementing.
/// This technique is similar to adding a trait bound to the trait.
use std::fmt;
use fmt::{ Display };

pub trait OutlinePrint: Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();

    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

pub struct Pointt {
  x: i32,
  y: i32
}

// `advanced_traits::Pointt` doesn't implement `std::fmt::Display
impl Display for Pointt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl OutlinePrint for Pointt {}

// 6. Using the Newtype Pattern to Implement External Traits on External Types
/// `orphan rule` and `newtype pattern`

/// As an example, let’s say we want to implement Display on Vec<T>, which the 
/// orphan rule prevents us from doing directly because the Display trait and 
/// the Vec<T> type are defined outside our crate. We can make a Wrapper struct 
/// that holds an instance of Vec<T>; then we can implement Display on Wrapper 
/// and use the Vec<T> value

/// If we wanted the new type to have every method the inner type has, 
/// implementing the Deref trait on the Wrapper to return the inner type would be a solution.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_trait() {
    let p = Point { x: 1, y: 1 } + Point { x: 2, y: 3 };
    assert_eq!(p, Point { x: 3, y: 4 });
  }

  #[test]
  fn set_default_type_rhs() {
    let a = Millimeters(32);
    let b = Meters(2);

    assert_eq!(a + b, Millimeters(2032));
  }

  #[test]
  fn area() {
    let s = Square { x: 0.0, y: 0.0, side: 12.0 };
    assert_eq!(s.area(), 144.0);
  }

  #[test]
  fn tait_use_fully_qualified_syntax() {
    assert_eq!(Dog::baby_name(), String::from("Spot"));
    assert_eq!(<Dog as Animal>::baby_name(), String::from("Puppy"));
    // assert!(Animal::baby_name()); // this will cause error `type annotations required`
  }

  #[test]
  fn super_trait() {
    let p = Pointt { x: 1, y: 2 };
    p.outline_print();
  }

  #[test]
  fn new_type() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    assert_eq!(format!("{}", w), "[hello, world]");
  }
}