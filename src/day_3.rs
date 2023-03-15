use std::fmt::{Debug, Display};

pub fn day_3_run() {
    greet_example();
    player_example();
    default_method_example();
    example_common_traits();
}

// Types that implement a given trait
// may be of different sizes.
// This makes it impossible to have
// things like Vec<Greet>.
trait Greet {
    fn say_hello(&self);
}

struct Cat;
struct Dog {
    name: String
}

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Cane {}", self.name);
    }
}

impl Greet for Cat {
    fn say_hello(&self) {
        println!("Gatto")
    }
}

fn greet_example() {
    let pets: Vec<Box<dyn Greet>> = vec![
        Box::new(Dog { name: String::from("Fido") }),
        Box::new(Cat)
    ];
    for pet in pets {
        pet.say_hello();
    }
}

// Player example
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn player_example() {
    // create player with default value for each field
    let p1 = Player::default();
    let p2 = p1.clone();
    println!("Is {:?}\nequal to {:?}?\nThe answer is {}!", &p1, &p2,
             if p1 == p2 { "yes" } else { "no" });
}


// Default methods example
trait Equals {
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equal(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

fn default_method_example() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));
}

// Common traits
// 1. iterators
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
// I can replace T with a specific type and implement trait for specific type
impl<T> Point<T> {
    fn first(&self) -> &T {
        &self.x
    }
}

// traits bound
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn example_common_traits() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for (i, n) in fib.enumerate().take(5) {
        println!("fib({i}): {n}");
    }

    // create collection from iterator
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();
    prime_squares.iter().for_each(|i| print!("{},", i));

    // generics
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("p.x = {}", integer.first());

    // trait bound
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
    let many = add_42_millions(42_i8);
    println!("{many}");
    // not work: let many_more = add_42_millions(20f);
    let xs = vec![123, "Hello"];

}
