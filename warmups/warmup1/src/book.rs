pub struct Book {
    title: String,
    number_of_pages: u32
}


impl Book {
    // A constructor to create a new Book
    pub fn new(title: String, number_of_pages: u32) -> Book {
        Book {
            title,
            number_of_pages
        }
    }

    // An instance method
    pub fn description(&self) {
        println!("'{}', {} pages", self.title,  self.number_of_pages);
    }
}