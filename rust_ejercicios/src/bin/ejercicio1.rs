fn main() {
    // La función main es el punto de entrada de un programa en Rust.
    // Es donde comienza la ejecución del programa.
    // Declaramos una variable inmutable llamada 'nombre' y le asignamos el valor "Rodrigo".
    // En Rust, por defecto, las variables son inmutables, es decir, no pueden cambiar su valor.
    let nombre = "Rodrigo";
    
    // Declaramos una variable mutable llamada 'edad' con valor inicial 30.
    // Usamos 'mut' para indicar que esta variable puede cambiar su valor posteriormente.
    let mut edad = 30;
    
    // println! es una macro que imprime texto en la consola.
    // Aquí usamos {} como marcador de posición para insertar valores de variables.
    // En este caso, imprimimos un saludo con el nombre y la edad.
    println!("Hola, {}! Tienes {} años.", nombre, edad);
    
    // Incrementamos la variable 'edad' en 1.
    // Esto solo es posible porque 'edad' fue declarada como mutable.
    edad += 1;
    
    // Imprimimos la edad actualizada.
    println!("El próximo año tendrás {} años.", edad);

    // Declaramos una variable 'pi' de tipo flotante de 64 bits (f64).
    // Es importante especificar el tipo para valores decimales cuando queremos precisión.
    let pi: f64 = 3.14159;
    
    // Declaramos una variable booleana 'activo' que puede ser true o false.
    let activo: bool = true;
    
    // Imprimimos los valores de 'pi' y 'activo' usando println! y la interpolación de variables.
    println!("Valor de pi: {}, activo: {}", pi, activo);

    // Estructura de control if para ejecutar código condicionalmente.
    // Si la edad es mayor que 18, se imprime que eres mayor de edad.
    if edad > 18 {
        println!("Eres mayor de edad");
    }
    
    // Bucle for que itera desde 1 hasta 5 inclusive.
    // El rango 1..=5 incluye el número 5.
    // En cada iteración, imprime el número actual.
    for i in 1..=5 {
        println!("Número: {}", i);
    }
}
