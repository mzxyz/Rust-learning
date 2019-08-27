pub mod cycle_reference {
  /// Rust’s memory safety guarantees make it difficult, but not impossible, 
  /// to accidentally create memory that is never cleaned up (known as a memory leak).
  /// memory leaks are memory safe in Rust. We can see that Rust allows memory 
  /// leaks by using Rc<T> and RefCell<T>: it’s possible to create references 
  /// where items refer to each other in a cycle. This creates memory leaks 
  /// because the reference count of each item in the cycle will never reach 0, 
  /// and the values will never be dropped.
  use std::rc::Rc;
  use std::cell::RefCell;
  use CycList::{Cons, Nil};
  
  #[derive(Debug)]
  pub enum CycList {
    Cons(i32, RefCell<Rc<CycList>>),
    Nil
  }

  impl CycList {
    pub fn tail(&self) -> Option<&RefCell<Rc<CycList>>> {
      match self {
        Cons(_, item) => Some(item),
        Nil => None
      }
    }
  }
}