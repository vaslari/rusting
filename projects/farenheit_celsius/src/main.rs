use std::io;

fn main() {
    loop {
        // Preguntar por la fuente de los grados
        println!(
            "Selecciona la unidad de origen de los grados (f: Fahrenheit, c: Celsius, k: Kelvin): "
        );

        let mut source_unit = String::new();
        io::stdin()
            .read_line(&mut source_unit)
            .expect("Error al leer la línea");

        let source_unit = source_unit.trim().to_lowercase();

        // Preguntar por el número de grados
        println!("Introduce el número de grados: ");

        let mut degrees = String::new();
        io::stdin()
            .read_line(&mut degrees)
            .expect("Error al leer la línea");

        let degrees: f64 = degrees
            .trim()
            .parse()
            .expect("Por favor, introduce un número válido");

        // Realizar la conversión
        match source_unit.as_str() {
            "f" => {
                let celsius = fahrenheit_to_celsius(degrees);
                let kelvin = celsius_to_kelvin(celsius);

                println!("{:.2} °F son {:.2} °C y {:.2} K", degrees, celsius, kelvin);
            }
            "c" => {
                let fahrenheit = celsius_to_fahrenheit(degrees);
                let kelvin = celsius_to_kelvin(degrees);

                println!(
                    "{:.2} °C son {:.2} °F y {:.2} K",
                    degrees, fahrenheit, kelvin
                );
            }
            "k" => {
                let celsius = kelvin_to_celsius(degrees);
                let fahrenheit = celsius_to_fahrenheit(celsius);

                println!(
                    "{:.2} K son {:.2} °C y {:.2} °F",
                    degrees, celsius, fahrenheit
                );
            }
            _ => {
                println!("Unidad no válida. Por favor, selecciona 'f' para Fahrenheit, 'c' para Celsius o 'k' para Kelvin.");
            }
        }

        // Preguntar si desea realizar otra conversión
        println!("¿Quieres realizar otra conversión? (s/n): ");

        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Error al leer la línea");

        if response.trim().to_lowercase() != "s" {
            break;
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
}

fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}
