mod fibonacci;
use fibonacci::Fib;

use std::{io, str::FromStr};
fn main() {
    let n: usize = loop {   // usize because n is an index
        let entry: i64 = leer("Choose a fibonacci index (non-negative integer):");
        match usize::try_from(entry) {
            Ok(num) => break num,
            Err(_)  => println!("Please enter a non-negative integer."),
        }
    };

    match Fib::new().nth(n) {  
        Some(f) => println!("The {n}-th Fibonacci number is {f}"),
        None    => println!("Fibonacci({n}) overflows u128."),
    }
}

fn leer<T: FromStr>(prompt: &str) -> T {
    loop {
        println!("{prompt}");
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error reading entry.");

        match entrada.trim().parse() {
            Ok(valor) => return valor,
            Err(_)    => println!("Invalid input, please try again."),
        }
    }
}
