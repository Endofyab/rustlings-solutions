// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option { // for -> if let Some(x)
        res += x;
    }
    println!("{}", res);
}
/* original program 

    let mut res = 42;
    let option = Some(12);
    for x in option {...}....

error: for loop over an `Option`. This is more readably written as an `if let` statement
 --> clippy2.rs:9:14
  |
9 |     for x in option {
  |              ^^^^^^
  |
  = note: `-D for-loops-over-fallibles` implied by `-D warnings`
help: to check pattern in a loop use `while let`
  |
9 |     while let Some(x) = option {
  |     ~~~~~~~~~~~~~~~ ~~~
help: consider using `if let` to clear intent
  |
9 |     if let Some(x) = option {
  |     ~~~~~~~~~~~~ ~~~

error: could not compile `clippy2` due to previous error

 */