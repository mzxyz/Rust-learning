enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("Call Message");
  }
}

/// options
enum Option<T> {
  Some(T), //  <T> means the Some variant of the Option enum can hold one piece of data of any type.
  None,
}

fn option_ex() {
  let _some_number = Some(5);
  let _some_string = Some("a string");

  // If we use None rather than Some, we need to tell Rust what type of Option<T> we have, 
  // because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.
  // you have to convert an Option<T> to a T before you can perform T operations with it. 
  let _absent_number: Option<i32> = None;
}

/// the match control flow operator
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 3,
    Coin::Dime => 5,
    Coin::Quarter(state) => {
      println!("{:?}", state);
      24
    },
    // The _ pattern will match any value
    _ => (),
  }
}

/// match with option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

/// if let is a syntax sugar for match that runs code when the value matches one pattern and then ignores all other values.
fn if_let(coin: Coin) {
  let mut count = 0;
  match coin {
    Coin::Quarter(state) => println!("It is quarter"),
    _ => count += 1,
  }

  // same with above code
  if let Coin::Quarter(state) = coin {
    println!("It is quarter");
  } else {
    count += 1;
  }
}

fn main() {
  let m = Message::Write(String::from("Hello"));
  m.call();

  option_ex();
  let five = Some(5);
  let _six = plus_one(five);
  let _none = plus_one(None);
}
