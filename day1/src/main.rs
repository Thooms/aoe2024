use common::mean;

fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    if let Some(result) = mean(&numbers) {
        println!("Day 1 - Mean of numbers: {}", result);
    }
} 