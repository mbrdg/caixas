use caixas::Box;

/// Prints a framed `Box` with `Hello, World!` written.
fn main() {
    print!("{}", Box::from("Hello, World!").framed())
}
