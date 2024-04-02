use std::sync::{Arc, Mutex};
use std::thread;

fn print_mutex_value() {
    let m = Mutex::new(123);
    let num = m.lock().unwrap();
    println!("num:  {}", num);
}

fn mutate_mutex() {
    let m = Mutex::new(123);
    let mut num = m.lock().unwrap();
    *num += 1;
    println!("num:  {}", num);
}

fn mutex_acquired_twice() {
    let m = Mutex::new(999);

    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }

    let num = m.lock().unwrap();
    println!("num:  {}", num);
}

fn mutex_multiple_threads() {
    let last_thread = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 1..10 {
        let last_thread = Arc::clone(&last_thread);
        let handle = thread::spawn(move || {
            let mut num = last_thread.lock().unwrap();
            *num = i;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Last thread to set the value was: {}",
        *last_thread.lock().unwrap()
    );
}

fn main() {
    print_mutex_value();
    mutate_mutex();
    mutex_acquired_twice();
    mutex_multiple_threads();
}
