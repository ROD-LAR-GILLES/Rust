fn main() {
    let s1 = String::from("Rust");
    let s2 = s1;
    println!("s2 tiene el valor: {}", s2);

    let s3 = String::from("Propiedad");
    imprime_longitud(&s3);
    println!("s3 sigue siendo válido: {}", s3);
}

fn imprime_longitud(texto: &String) {
    println!("Longitud de '{}': {}", texto, texto.len());
}
