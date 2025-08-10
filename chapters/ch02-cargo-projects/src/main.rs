use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // ========================
    // 1. Importación de dependencias
    // ========================
    println!("¡Adivina el número!");

    // ========================
    // 2. Mensaje de bienvenida
    // ========================
    // Generar número secreto entre 1 y 100
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    // ========================
    // 3. Bucle para seguir pidiendo intentos hasta que el usuario acierte
    // ========================
    loop {
        println!("Por favor, introduce tu número:");

        // ========================
        // 4. Lectura de entrada del usuario
        // ========================
        let mut suposicion = String::new();

        io::stdin()
            .read_line(&mut suposicion)
            .expect("Error al leer la línea");

        // ========================
        // 5. Conversión de la entrada a número
        // ========================
        let suposicion: u32 = match suposicion.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce un número válido.");
                continue;
            }
        };

        println!("Tu suposición: {suposicion}");

        // ========================
        // 6. Comparación con el número secreto y mensajes según el resultado
        // ========================
        match suposicion.cmp(&numero_secreto) {
            Ordering::Less => println!("Demasiado pequeño!"),
            Ordering::Greater => println!("Demasiado grande!"),
            Ordering::Equal => {
                println!("¡Acertaste!");
                break;
            }
        }
    }
}