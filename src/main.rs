mod library;
mod day_2_morning;
mod day_2_morning_excercise;
mod day_3_morning;
mod day_3_morning_exercises;

use crate::day_2_morning::day2morning_entry;
use crate::day_2_morning_excercise::day_2_run_morning_exercise;
use crate::day_3_morning::day_3;
use crate::library::{Book, Library};

fn say_hello(name: String) {
    println!("Hello {name}")
}

#[derive(Clone, Debug)]
struct Point(i32, i32, String);

fn add(p1: &Point, p2: &Point) -> Point {
    let p = Point(p1.0 + p2.0, p1.1 + p2.1, String::from("ciao"));
    println!("&p.0: {:p}", &p.0);
    p
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);

    // copying and cloning

    // cause point derive copy when assign p2 from p1, p1 is copied, like integers
    let p1 = Point(3, 4, String::from("ciao"));
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    let p3 = add(&p1, &p2);
    println!("&p3.0: {:p}", &p3.0);
    println!("{p1:?} + {p2:?} = {p3:?}");

    // library exercise
    let mut library = Library::new();
    println!("Our library is empty: {}", library.is_empty());
    let favorite_book = Book::new("Lord of the Rings", 1954);
    println!("Our favorite book {favorite_book} should go in the library");
    library.add_book(favorite_book);
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    library.print_books();
    match library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }
    for book in library {
        println!("Book iter {book}")
    }

    day2morning_entry();
    day_2_run_morning_exercise();

    day_3();
}
