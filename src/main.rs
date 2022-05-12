// TODO(elsuizo:2022-04-08): lo que quiero hacer es varias tareas que se ejecuten de manera
// periodica

fn my_task(name: &str) {
    println!("hola desde la tarea: {}", name);
}

#[tokio::main]
async fn main() {
    let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(1).to_std().unwrap());
    loop {
        // Wait for the next interval tick
        interval_timer.tick().await;
        // tokio::spawn(async { do_my_task().await; }); // For async task
        tokio::task::spawn_blocking(|| my_task("my_task")); // For blocking task
    }
}
