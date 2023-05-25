// lifetimes3.rs
// Lifetimes are also needed when structs hold references.

// struct Book {
//     author: &str,
//     title: &str,
// }
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
