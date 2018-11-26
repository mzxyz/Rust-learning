/// By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear.
/// Methods let you specify the behavior that instances of your structs have.
/// Associated functions let you namespace functionality that is particular to your struct without having an instance available.

/// struct
struct User {
    name: String,
    email: String,
    account: u64,
    active: bool,
}

// instance of User
fn new_user() -> User {
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user1 = User {
        email: String::from("ddd@gmail.com"),
        name: String::from("Alex"),
        account: 33,
        active: true,
    };

    user1.account = 3434;
    println!("{}", user1.account);

    user1
}

/// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/// example - rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


/// method
/// methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object)
/// their first parameter is always self, which represents the instance of the struct the method is being called on.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn cycle(&self) -> u32 {
        (self.width + self.height) * 2
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


/// associated function
/// Another useful feature of impl blocks is that we’re allowed to define functions within
/// impl blocks that don’t take self as a parameter. 
/// These are called associated functions because they’re associated with the struct. They’re still functions, not methods
/// because they don’t have an instance of the struct to work with
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


/// borrow other than ownership
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


fn main() {
   let _user1 = new_user();

   let _black = Color(0, 0, 0);
   let _origin = Point(0, 0, 0);

   let rect1 = Rectangle {
       width: 30,
       height: 50,
   };

   let area_size = area(&rect1);

   // call method
   // when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method.
   // In other words, the following are the same:
   let cycle_size = rect1.cycle();

   println!("The area of rectangle is {}", area_size);
   println!("The cycle size of rectangle is {}", cycle_size);
   println!("{:#?}", rect1);

   // call associate function
   Rectangle::square(40);
}
