use std::io;

fn main() {
    for i in 1..10 {
        println!("{}", factorial(i))
    }
}

fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial(n-1)
    }
}
