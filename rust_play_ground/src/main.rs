use std::time::{Duration, Instant};
fn main() {
    let n:usize = 45;
    let start = Instant::now();
    println!("{}", fibonacci(n));
    let duration = start.elapsed();
    println!("Number given: {}",n);
    println!("Time to finish: {:?}", duration);
}
fn fibonacci(n:usize) -> usize {
    if n <= 1 {
        return n;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}
