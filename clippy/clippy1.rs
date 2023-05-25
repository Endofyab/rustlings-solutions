// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a hint.

use std::f32;

fn main() {
    //let pi = 3.14f32; clippy wants us to change this
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}

/*
error: approximate value of `f32::consts::PI` found
  --> clippy1.rs:14:14
   |
14 |     let pi = 3.14f32;
   |              ^^^^^^^
   |
   = help: consider using the constant directly
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
   = note: `#[deny(clippy::approx_constant)]` on by default

error: could not compile `clippy1` due to previous error

Rustlings hint: 
Rust stores the highest precision version of any long or infinite precision
mathematical constants in the Rust standard library.
https://doc.rust-lang.org/stable/std/f32/consts/index.html

We may be tempted to use our own approximations for certain mathematical constants,
but clippy recognizes those imprecise mathematical constants as a source of
potential error.
See the suggestions of the clippy warning in compile output and use the
appropriate replacement constant from std::f32::consts...
*/