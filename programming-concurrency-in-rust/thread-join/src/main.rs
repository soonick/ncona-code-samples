use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("The spawned thread");
    });

    handle.join().unwrap();
}
