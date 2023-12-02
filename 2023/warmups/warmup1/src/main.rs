mod book;
use book::Book;

fn main() {
    let book:Book = Book::new("Assassinio a Venezia".parse().unwrap(), 120);
    book.description();

    //Immutable string
    let _string_literal = "Ciao!";
    
    //Mutable string
    let _string = String::from("Ciao!");
}
