// 1. Using the Newtype Pattern for Type Safety and Abstraction
/// * statically enforcing that values are never confused and 
/// indicating the units of a value. 

/// * newtype pattern is in abstracting away some implementation 
/// details of a type: the new type can expose a public API 
/// that is different from the API of the private inner type 

/// if we used the new type directly to restrict the available functionality
/// * The newtype pattern is a lightweight way to achieve encapsulation to 
/// hide implementation details

// 2. Creating Type Synonyms with `Type Aliases`
/// * A type alias makes this code more manageable by reducing the repetition
/// * Type aliases are also commonly used with the Result<T, E> type for reducing repetition
use std::io::Error;
use std::result;
use std::fmt;

type Result<T> = result::Result<T, Error>;

pub trait Write {
  fn write(&mut self, buf: &[u8]) -> Result<usize>;
  fn flush(&mut self) -> Result<()>;
 }

// 3. The Never Type that Never Returns
/// Rust has a special type named ! that’s known in type 
/// theory lingo as the empty type because it has no values. 
/// We prefer to call it the never type because it stands 
/// in the place of the return type when a function will never return.
fn bar() -> ! {};

// 4. Dynamically Sized Types and the Sized Trait
/// Rust needs to know how much memory to allocate for any 
/// value of a particular type, and all values of a type 
/// must use the same amount of memory.

/// The slice data structure (&str) stores the starting position and the length of the slice
/// So although a &T is a single value that stores the memory address of where the T is located, 
/// a &str is two values: the address of the str and its length.
/// In general, this is the way in which dynamically sized types are used in Rust: they have 
/// an extra bit of metadata that stores the size of the dynamic information. 

/// The golden rule of dynamically sized types is that we must always put values of 
/// dynamically sized types behind a pointer of some kind.

/// To work with DSTs, Rust has a particular trait called the `Sized trait`
/// to determine whether or not a type’s size is known at compile time. 
fn generic<T>(t: T) {};
// same with:
fn generic<T: Sized>(t: T) {};

/// By default, generic functions will work only on types that have
/// a known size at compile time. However, you can use the following
/// special syntax to relax this restriction:
fn generic<T: ?Sized>(t: &T) {};

/// A trait bound on ?Sized is the opposite of a trait bound on Sized: 
/// we would read this as “T may or may not be Sized.” This syntax is 
/// only available for Sized, not any other traits.

/// Also note that we switched the type of the t parameter from T to &T.
/// Because the type might not be Sized, we need to use it behind some 
/// kind of pointer. In this case, we’ve chosen a reference.