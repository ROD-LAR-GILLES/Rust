fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let suma: i32 = numeros.iter().sum();
    println!("Suma: {}", suma);

    match dividir(10, 2) {
        Ok(res) => println!("Resultado 10/2: {}", res),
        Err(e) => println!("Error: {}", e),
    }
    match dividir(5, 0) {
        Ok(res) => println!("Resultado 5/0: {}", res),
        Err(e) => println!("Error: {}", e),
    }
}

fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("División por cero"))
    } else {
        Ok(a / b)
    }
}
