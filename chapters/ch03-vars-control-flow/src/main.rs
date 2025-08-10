fn main() {
    // ========================
    // 1. Variables inmutables
    // ========================
    let x = 5; // Por defecto, las variables en Rust son inmutables
    println!("El valor de x es: {x}");

    // ========================
    // 2. Variables mutables
    // ========================
    let mut y = 10; // "mut" permite cambiar el valor después
    println!("El valor inicial de y es: {y}");
    y = 20;
    println!("El nuevo valor de y es: {y}");

    // ========================
    // 3. Constantes
    // ========================
    const MAX_POINTS: u32 = 100_000; // Constantes siempre requieren tipo
    println!("Constante MAX_POINTS: {MAX_POINTS}");

    // ========================
    // 4. Shadowing (sombras)
    // ========================
    let z = 2;
    let z = z + 1; // Crea una nueva variable "z" sobre la anterior
    let z = z * 2; // Puede cambiar el tipo si fuera necesario
    println!("Valor de z después de shadowing: {z}");

    // ========================
    // 5. Control de flujo con if
    // ========================
    if z > 5 {
        println!("z es mayor que 5");
    } else {
        println!("z es 5 o menor");
    }

    // ========================
    // 6. Bucle for
    // ========================
    println!("Bucle for:");
    for numero in 1..4 {
        println!("Número: {numero}");
    }

    // ========================
    // 7. Bucle while
    // ========================
    println!("Bucle while:");
    let mut contador = 3;
    while contador != 0 {
        println!("{contador}!");
        contador -= 1;
    }
    println!("¡Despegue!");

    // ========================
    // 8. Bucle loop con break
    // ========================
    let mut contador2 = 0;
    let resultado = loop {
        contador2 += 1;
        if contador2 == 5 {
            break contador2 * 2; // Rompe el loop y retorna un valor
        }
    };
    println!("El resultado del loop es: {resultado}");
}