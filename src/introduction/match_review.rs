pub fn match_review() {
    let number = 13;

    println!("NUMBER: {}", number);
    match number {
        1 => println!("ONE IS ONE BTW"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("PRIME"),
        14..=19 => println!("what is that?"),
        _ => println!("HuH?"),
    }
}
