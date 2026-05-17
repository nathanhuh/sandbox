use std::io::{self, Write};

fn main() {
    print!("What is the first number? ");
    io::stdout().flush().unwrap();
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();
    let first_num = first_input.trim().parse::<i32>().unwrap();

    print!("What is the second number? ");
    io::stdout().flush().unwrap();
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).unwrap();
    let second_num = second_input.trim().parse::<i32>().unwrap();

    let sum = first_num + second_num;
    let negate = first_num - second_num;
    let multiple = first_num * second_num;
    let division = first_num / second_num;
    println!(
        "{first_num} + {second_num} = {sum}\n{first_num} - {second_num} = {negate}\n{first_num} * {second_num} = {multiple}\n{first_num} / {second_num} = {division}"
    );
}
