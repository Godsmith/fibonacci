use cached::proc_macro::cached;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match args[1].trim().parse::<u128>() {
            Ok(n) => {
                println!("{}", fibonacci(n))
            }
            Err(_) => println!("not number"),
        }
    } else {
        println!("Wrong number of arguments.")
    }
}

/// Overflows for large input
#[cached]
fn fibonacci(n: u128) -> u128 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
