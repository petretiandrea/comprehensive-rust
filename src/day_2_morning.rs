
pub fn day2morning_entry() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");

    let tmp = Person {
        ..Default::default()
    };
    println!("{tmp:?}");

    let tmp = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
    println!("{tmp:?}");

    println!("You got: {:?}", flip_coin());

    let press = WebEvent::KeyPress('x');
    inspect(press);

    // pattern matching
    let input = 'x';
    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }
    match_foo()
}

// structs without name, if its unimportant, also use to make type alias
struct Point(i32, i32);
struct PoundOfForce(f64);
struct Newtons(f64);

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
    // use Self as it is interchangeable
    fn newWithSelf(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

// enums
fn generate_random_number() -> i32 {
    4  // Chosen by fair dice roll. Guaranteed to be random.
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    return if random_number % 2 == 0 {
        CoinFlip::Heads
    } else {
        CoinFlip::Tails
    }
}

enum WebEvent {
    PageLoad,                 // Variant without payload
    KeyPress(char),           // Tuple struct variant
    Click { x: i64, y: i64 }, // Full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

// pattern match destructuring
struct Foo {
    x: (u32, u32),
    y: u32,
}

fn match_foo() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}