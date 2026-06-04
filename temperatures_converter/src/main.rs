use std::{io, str::FromStr};
fn main() {
    println!("Fahrenheit to Celsius - [1]");
    println!("Celsius to Fahrenheit - [2]");

    let option: u32 = loop {
        let n: u32 = leer("Choose conversion (1 or 2):");
        if n == 1 || n == 2 {
            break n;
        }
        println!("Tenés que elegir 1 o 2.");
    };

    match option {
        1 => {
            let temp: f64 = leer("Enter Farenheit:");
            println!("{:.2} °C", f_to_c(temp));
        },
        2 => {
            let temp: f64 = leer("Enter Celsius:");
            println!("{:.2} °F", c_to_f(temp));
        },
        _ => unreachable!(), 
    };
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn leer<T: FromStr>(prompt: &str) -> T {
    loop {
        println!("{prompt}");
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");

        match entrada.trim().parse() {
            Ok(valor) => return valor,
            Err(_) => println!("Entrada inválida, probá de nuevo."),
        }
    }
}