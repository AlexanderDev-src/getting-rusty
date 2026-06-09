use std::io::{self, Write};
pub fn pattern() {
    print!("Enter number to draw the pattern: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Errors");

    let numbers: i32 = input.trim().parse().expect("Please Enter number");

    for i in 1..=numbers {
        for j in i..numbers {
            print!(" ");
        }
        for j in 1..=(2 * i - 1) {
            print!("*");
        }
        println!();
    }
}
