// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("What't in my_option is: {:#?}", my_option); //my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!                       value_a = value_b; value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b); // use this instad 
    println!("value a: {}; value b: {}", value_a, value_b);
}

/* error message

error: possibly missing a comma here
  --> clippy3.rs:14:19
   |
14 |         -1, -2, -3
   |                   ^
   |
   = note: to remove this lint, add a comma or write the expr in a single line
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#possible_missing_comma
   = note: `#[deny(clippy::possible_missing_comma)]` on by default

error: this call to `unwrap()` will always panic
  --> clippy3.rs:10:9
   |
9  |     if my_option.is_none() {
   |        ------------------- because of this check
10 |         my_option.unwrap();
   |         ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#panicking_unwrap
   = note: `#[deny(clippy::panicking_unwrap)]` on by default

error: this looks like you are trying to swap `value_a` and `value_b`
  --> clippy3.rs:25:5
   |
25 | /     value_a = value_b;
26 | |     value_b = value_a;
   | |_____________________^ help: try: `std::mem::swap(&mut value_a, &mut value_b)`
   |
   = note: or maybe you should use `std::mem::replace`?
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#almost_swapped
   = note: `#[deny(clippy::almost_swapped)]` on by default

error: this let-binding has unit value
  --> clippy3.rs:19:5
   |
19 |     let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `vec![1, 2, 3, 4, 5].resize(0, 5);`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#let_unit_value
   = note: `-D clippy::let-unit-value` implied by `-D warnings`

error: could not compile `clippy3` due to 4 previous errors



 */