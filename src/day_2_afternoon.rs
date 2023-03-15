
pub fn day_2() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");

    outer::inner::public()
}

// box is used to defer pointer from stack to type
// for example recursive data structure: rust compiler cannot establish the size of
// struct at compile time, so by using Box compiler can determine a size, but the data
// will be allocated to heap
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// modules
mod outer {
    fn private() {
        println!("outer::private")
    }

    pub fn public() {
        print!("outer::public");
    }

    pub mod inner {
        fn private() {
            println!("outer::inner:private")
        }

        pub fn public() {
            println!("outer::inner::public");
            self::private();
            super::private();
        }
    }
}