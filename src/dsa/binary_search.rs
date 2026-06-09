use rand::Rng;
use std::io::{self, Write};
fn Binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid
        }
    }
    None
}

pub fn binary_search_run() {
    let mut rng = rand::thread_rng();
    let mut number: Vec<i32> = (0..=20).map(|_| rng.gen_range(0..=1000)).collect();
    number.sort();
    println!("{:?}", number);

    print!("Enter your target to find: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Errors");

    let target: i32 = input.trim().parse().expect("Please Enter number");

    match Binary_search(&number, target) {
        Some(index) => println!("Find target: {} in Index {}", target, index),
        None => println!("Not Found"),
    }
}
