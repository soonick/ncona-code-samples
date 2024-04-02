use std::thread;

fn main() {
    thread::spawn(|| {
        println!("The spawned thread");
    });
}
