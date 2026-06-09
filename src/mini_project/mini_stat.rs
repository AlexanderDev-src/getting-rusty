fn count(data: &[i32]) -> usize {
    data.len()
}
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}
fn mean(data: &[i32]) -> Option<f64> {
    let sum: i32 = data.iter().sum();
    Some(sum as f64 / data.len() as f64)
}
fn median(data: &[i32]) -> Option<f64> {
    let mut sorted = data.to_vec();
    sorted.sort();
    let mid = sorted.len() / 2;
    if sorted.len() / 2 == 0 {
        Some(sorted[mid - 1] as f64 + sorted[mid] as f64 / 2.0)
    } else {
        Some(sorted[mid] as f64 / 2.0)
    }
}
fn above_avg(data: &[i32]) -> Vec<i32> {
    let sum: i32 = data.iter().sum();
    let avg = sum as f64 / data.len() as f64;
    //Note .copied make &i32 -> i32
    let above_avg: Vec<i32> = data.iter().copied().filter(|&x| x as f64 > avg).collect();
    above_avg
}

pub fn start_stat() {
    let data = vec![10, 20, 30, 20, 50, 40, 20, 10];
    println!("count     : {}", count(&data));
    println!("sum       : {}", sum(&data));
    println!("mean      : {:?}", mean(&data).unwrap());
    println!("median    : {:?}", median(&data).unwrap());
    println!("max       : {}", data.iter().max().unwrap());
    println!("min       : {}", data.iter().min().unwrap());
    println!("above avg : {:?}", above_avg(&data));
}
