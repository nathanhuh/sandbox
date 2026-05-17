use chrono::{Datelike, Utc};
use std::io::{self, Write};

fn main() {
    let now = Utc::now();
    let (_, curr_year) = now.year_ce();

    print!("What is your current age? ");
    io::stdout().flush().unwrap();
    let mut curr_age_str = String::new();
    io::stdin().read_line(&mut curr_age_str).unwrap();
    let curr_age: u32 = curr_age_str.trim().parse::<u32>().unwrap();

    print!("At what age would you like to retire? ");
    io::stdout().flush().unwrap();
    let mut retire_age_str = String::new();
    io::stdin().read_line(&mut retire_age_str).unwrap();
    let retire_age: u32 = retire_age_str.trim().parse::<u32>().unwrap();

    let years_to_work: u32 = retire_age - curr_age;
    let retire_year: u32 = curr_year + years_to_work;

    println!("You have {years_to_work} years left until you can retire.");
    println!("It's {curr_year}, so you can retire in {retire_year}.");
}
