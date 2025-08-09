use std::thread;
use std::time::Duration;

fn main() {
    let manejador = thread::spawn(|| {
        for i in 1..=4 {
            println!("Desde hilo: {}", i);
            thread::sleep(Duration::from_millis(400));
        }
    });

    for i in 1..=4 {
        println!("Desde main: {}", i);
        thread::sleep(Duration::from_millis(400));
    }

    manejador.join().expect("El hilo secundario falló");
}
