// lifetimes2.rs
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk
// of going out of scope before it is used. Remember, references are borrows
// and do not own their own data. What if their owner goes out of scope?
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.


// fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//How lifetime'a works is that it gets the smallest lifetime of the two variables and then kills both 


//Explanation: error is in main, borrowed value doesn't live enough 
// let string1, result 
// new scope { 
//  let string2 = String::from("sdfsf")
//  result = longest(string1.as_str(), string2.as_str() } end of scope 
// println!("{}", result)

//We could remove the scope, move the string2 declaration so it lives until string1 dies, 
//or move println! into the block 

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz"); // change : moved string2 out of the scope, it has same lifetime as 1 now 
    {
        result = longest(string1.as_str(), string2.as_str()); 
    }
    println!("The longest string is '{}'", result);
}
