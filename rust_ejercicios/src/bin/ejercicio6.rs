use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let tarea1 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Tarea 1 lista");
    };
    let tarea2 = async {
        println!("Tarea 2 lista");
    };
    tokio::join!(tarea1, tarea2);
}
