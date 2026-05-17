use std::io::{self, Write};

fn main() {
    print!("What is the input string? ");

    io::stdout().flush().expect("flush failed");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input has been wrong");
    input = input.trim().to_string();
    let input_length = input.len();

    let output = input + " has " + input_length.to_string().as_str() + " characters.";

    println!("{}", output);
}
