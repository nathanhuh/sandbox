use std::io::{self, Write};

fn main() {
    print!("What is your name?: ");
    io::stdout().flush().expect("flush failed");

    let mut name_input = String::new();
    io::stdin()
        .read_line(&mut name_input)
        .expect("Invalid name");
    name_input = name_input.trim().to_string();

    let result = "Hello, ".to_owned() + &name_input + ", nice to meet you!";

    println!("{}", result);
}
