use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        println!("The spawned thread");
    });

    thread::sleep(Duration::from_millis(1));
}
