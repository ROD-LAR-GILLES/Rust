fn main() {
    let nombre = "Rodrigo";
    let mut edad = 30;
    println!("Hola, {}! Tienes {} años.", nombre, edad);
    edad += 1;
    println!("El próximo año tendrás {} años.", edad);

    let pi: f64 = 3.14159;
    let activo: bool = true;
    println!("Valor de pi: {}, activo: {}", pi, activo);

    if edad > 18 {
        println!("Eres mayor de edad");
    }
    for i in 1..=5 {
        println!("Número: {}", i);
    }
}
