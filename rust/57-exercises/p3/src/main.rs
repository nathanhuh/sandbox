use std::io::{self, Write};

fn main() {
    print!("What is the quote? ");
    io::stdout().flush().unwrap();

    let mut quote = String::new();
    io::stdin().read_line(&mut quote).unwrap();
    quote = String::from(quote.trim());

    print!("Who said it? ");
    io::stdout().flush().unwrap();

    let mut source = String::new();
    io::stdin().read_line(&mut source).unwrap();
    source = String::from(source.trim());

    let output = source + " says, \"" + quote.as_str() + "\"";
    println!("{}", output);
}
