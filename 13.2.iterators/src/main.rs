

// Iterator doc: https://doc.rust-lang.org/std/iter/trait.Iterator.html

// Tterators, although a high-level abstraction, get compiled down to 
// roughly the same code as if you’d written the lower-level code yourself. 
// Iterators are one of Rust’s zero-cost abstractions, by which we mean 
// using the abstraction imposes no additional runtime overhead.

// zero-overhead principle: What you don’t use, you don’t pay for. 
// And further: What you do use, you couldn’t hand code any better.

// all iterators implement a trait named `Itarator` that is defined in the std
pub trait Iteratorr {
    // associated type: the type of the elements being iterated over.
    type Item;

    // Advances the iterator and returns the next value.
    // Returns None when iteration is finished.
    fn next(&mut self) -> Option<Self::Item>;
}

fn simple_iter() {
    let v1 = vec![1, 2, 3];

    // just create a iterator
    // If we want to create an iterator that takes ownership 
    // of v1 and returns owned values, we can call `into_iter` 
    // instead of iter. Similarly, if we want to iterate over 
    // mutable references, we can call `iter_mut` instead of iter.
    let v1_iter = v1.iter(); 

    for val in &v1 {
        println!("{}", val);
    }

    // Don't need to make v1_iter mutable when we used 
    // a for loop because the loop took ownership of 
    // v1_iter and made it mutable behind the scenes.
    for val in v1_iter {
        println!("{}", val);
    }
}

fn consuming_adaptor_sum() {
    let v1 = vec![1, 2, 3];

    // Methods that call next are called `consuming adaptors` -> `sum`
    // which takes ownership of the iterator and iterates through 
    // the items by repeatedly calling next, thus consuming the iterator
    // For example: `sum` will take the ownership of `v1_iter`
    let total: u32 = v1.iter().sum(); 
    assert_eq!(total, 6);
}


fn iterator_adaptor() {
    // Other methods defined on the Iterator trait, known as `iterator adaptors`, 
    // allow you to change iterators into different kinds of iterators.  You can 
    // chain multiple calls to iterator adaptors to perform complex actions in a readable way.
    // |map, filter ...|
    let v1 = vec![1, 2, 3];

    // Note that all iterators are lazy, you have to call one of the
    // `consuming adaptor` methods to get results from calls to iterator adaptors
    let v1_map_iter = v1.iter().map(|x| x*2);

    // consumes the iterator and collects the resulting values into a `collection data type`.
    let v2: Vec<_> = v1_map_iter.collect();
    assert_eq!(v2, vec![2, 4, 6]);
}

// demonstrate using closures that capture their environment
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

impl Shoe {
    fn new(size: u32, style: &str) -> Shoe {
        Shoe { size, style: style.to_string() }
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // The closure captures the `shoe_size` parameter from the environment.
    // We call `into_iter` to create an iterator that takes ownership of the vector. 
    shoes.into_iter()
        // adapt the iterator into a new iterator 
        .filter(|s| s.size == shoe_size)
        .collect()
}

// create own `iterators` with `Iterator` Trait
// Example: create an iterator that will only ever count from 1 to 5
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    simple_iter();
    iterator_adaptor();
    consuming_adaptor_sum();
}

#[test]
fn iterator_next() {
    let v1 = vec![1, 2, 3];

    // Note that we needed to make v1_iter mutable: calling 
    // the next method on an iterator changes internal state 
    // that the iterator uses to keep track of where it is in the sequence
    let mut v1_iter = v1.iter();

    // Some(val)  Option<&{integer}
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe::new(10, "Nike"),
        Shoe::new(13, "Puma"),
        Shoe::new(10, "Adidas")
    ];

    let my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        my_size, 
        vec![
            Shoe::new(10, "Nike"),
            Shoe::new(10, "Adidas"),
        ]);
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let counter = Counter::new();
    // skip(): creates an iterator that skips the first n elements
    let sum: u32 = counter.zip(Counter::new().skip(1))
        // ((1, 2), (2, 3), (3, 4), (4, 5))
        // zip() is often used to zip an infinite iterator
        // to a finite one. This works because the finite 
        // iterator will eventually return None, 
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}