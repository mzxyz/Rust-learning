pub mod advanced_traits {
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

  // 2. Default Generic Type Parameters and Operator Overloading
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
  ///  The new part is RHS=Self: this syntax is called default type parameters. The RHS generic 
  /// type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method. 
  /// 
  // trait Add<RHS=Self> {
  //   type Output;

  //   fn add(self, rhs: RHS) -> Self::Output;
  // }
  #[derive(Debug)]
  pub struct Millimeters(pub u32);

  #[derive(Debug)]
  pub struct Meters(pub u32);

  // You’ll use default type parameters in two main ways:
  // - To extend a type without breaking existing code
  // - To allow customization in specific cases most users won’t need
  impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + other.0 * 1000)
    }
  }
  
  // 3. Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

  // 4. Using Supertraits to Require One Trait’s Functionality Within Another Trait

  // 5. Using the Newtype Pattern to Implement External Traits on External Types
}

#[cfg(test)]
mod tests {
  use super::advanced_traits::*;

  #[test]
  fn add_trait() {
    let p = Point { x: 1, y: 1 } + Point { x: 2, y: 3 };
    assert_eq!(p, Point { x: 3, y: 4 });
  }

  #[test]
  fn add_rhs() {
    let a = Millimeters(32);
    let b = Meters(2);

    assert_eq!(a + b, Millimeters(2032));
  }
}