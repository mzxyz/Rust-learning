pub trait Messager {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messager> {
  messager: &'a T,
  value: usize,
  max: usize
}

impl <'a, T> LimitTracker<'a, T> where
  T: Messager
{
  fn new(messager: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messager,
      value: 0,
      max
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;
    let percentrage_of_max = self.value as f64 / self.max as f64;
    if percentrage_of_max > 0.75 && percentrage_of_max < 0.9 {
      self.messager.send("Warning: You've used up over 75% of your quota!");
    } else if percentrage_of_max >= 0.9 && percentrage_of_max < 1.0 {
      self.messager.send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentrage_of_max >= 1.0 {
      self.messager.send("Error: You are over your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell:RefCell;

  struct MockMessager {
    sent_messages: RefCell<Vec<String>>
  }

  impl MockMessager {
    fn new() -> MockMessager {
      MockMessager {
        sent_messages: RefCell::new(vec![])
      }
    }
  }

  impl Messager for MockMessager {
    fn send(&self, message: &str) {
      self.sent_messages.borrow_mut().push(String::from(message));
      println!("Messge: {}", message);
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning() {
    let mock_messager = MockMessager::new();
    let limit_tracker = LimitTracker::new(&mock_messager, 100);

    limit_tracker.set_value(80);
    assert_eq!(mock_messager.borrow().len(), 1);
  }
}