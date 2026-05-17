use std::io::{self, Write};

fn main() {
    print!("Enter a noun: ");
    io::stdout().flush().unwrap();
    let mut noun = String::new();
    io::stdin().read_line(&mut noun).unwrap();
    noun = String::from(noun.trim());

    print!("Enter a verb: ");
    io::stdout().flush().unwrap();
    let mut verb = String::new();
    io::stdin().read_line(&mut verb).unwrap();
    verb = String::from(verb.trim());

    print!("Enter an adjective: ");
    io::stdout().flush().unwrap();
    let mut adj = String::new();
    io::stdin().read_line(&mut adj).unwrap();
    adj = String::from(adj.trim());

    print!("Enter an adverb: ");
    io::stdout().flush().unwrap();
    let mut adv = String::new();
    io::stdin().read_line(&mut adv).unwrap();
    adv = String::from(adv.trim());

    println!("Do you {verb} your {adj} {noun} {adv}? That's cool, man.");
}
