fn main() {
    // ========================
    // 1. Introducción
    // ========================
    // Este es el primer programa en Rust: "Hello, world!"
    println!("Hello, world!");

    // ========================
    // 2. Mensaje adicional
    // ========================
    println!("Programa del capítulo 1 ejecutado correctamente.");

    // ========================
    // 3. Información del paquete
    // ========================
    println!("Nombre del paquete: {}", env!("CARGO_PKG_NAME"));
    println!("Versión del paquete: {}", env!("CARGO_PKG_VERSION"));

    // ========================
    // 4. Ruta del archivo
    // ========================
    println!("Este código está en el archivo: {}", file!());
}
