// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

//docs https://veykril.github.io/tlborm/ https://doc.rust-lang.org/book/ch19-06-macros.html
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };  // need semicolon. $val:expr Error: no rules expected this token in macro call, 
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
