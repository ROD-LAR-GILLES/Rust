struct Usuario {
    nombre: String,
    edad: u8,
    activo: bool,
}

enum Mensaje {
    Saludar(String),
    Despedir,
}

fn main() {
    let usuario = Usuario {
        nombre: String::from("Rodrigo"),
        edad: 30,
        activo: true,
    };
    println!("Usuario: {} ({})", usuario.nombre, usuario.edad);

    let mensaje = Mensaje::Saludar(String::from("¡Hola!"));
    procesar_mensaje(mensaje);
    procesar_mensaje(Mensaje::Despedir);
}

fn procesar_mensaje(m: Mensaje) {
    match m {
        Mensaje::Saludar(texto) => println!("{}", texto),
        Mensaje::Despedir => println!("¡Adiós!"),
    }
}
