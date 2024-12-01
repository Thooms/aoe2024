use common::mean;

fn main() {
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    if let Some(result) = mean(&numbers) {
        println!("Day 2 - Mean of numbers: {}", result);
    }
} 