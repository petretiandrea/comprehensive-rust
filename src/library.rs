use std::fmt::Formatter;

pub struct Library {
    books: Vec<Book>
}

pub struct Book {
    title: String,
    year: u16
}

impl Book {
    pub fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year
        }
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.books.len()
    }

    pub fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    pub fn print_books(&self) {
        println!("Library:");
        for book in &self.books {
            println!("{book}")
        }
    }

    pub fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
}
