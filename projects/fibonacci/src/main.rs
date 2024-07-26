use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;

fn main() {
    loop {
        // Preguntar por el valor de n
        println!("Introduce el valor de n para generar el n-ésimo número de Fibonacci: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la línea");
        let n: u32 = input
            .trim()
            .parse()
            .expect("Por favor, introduce un número válido");

        // Calcular el n-ésimo número de Fibonacci
        let fibonacci_number = fibonacci(n);
        println!(
            "El {}-ésimo número de Fibonacci es: {}",
            n, fibonacci_number
        );

        // Preguntar si desea generar otro número de Fibonacci
        println!("¿Quieres calcular otro número de Fibonacci? (s/n): ");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Error al leer la línea");
        if response.trim().to_lowercase() != "s" {
            break;
        }
    }
}

fn fibonacci(n: u32) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        _ => {
            let mut a: BigUint = BigUint::zero();
            let mut b: BigUint = BigUint::one();
            let mut c: BigUint;
            for _ in 2..=n {
                c = &a + &b;
                a = b;
                b = c;
            }
            b
        }
    }
}
