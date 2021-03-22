use std::ops::Deref;
use std::thread;
use std::time::Duration;

struct Book {
    name: &'static str,
    id: u32,
}

trait Printable {
    fn print(&self);
}

impl Printable for Book {
    fn print(&self) {
        println!("print:{}", self.name);
    }
}

#[test]
fn test() {
    let book = Book {
        name: "hello world",
        id: 32,
    };
    book.print();
}
