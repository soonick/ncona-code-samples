use core::time::Duration;
use std::sync::mpsc;
use std::thread;

fn single_message() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if tx.send("Hello world!").is_err() {
            println!("There was an error sending message");
        }
    });

    match rx.recv() {
        Ok(mes) => println!("Message: {}", mes),
        Err(err) => println!("There was an error receiving mesage. Error: {}", err)
    };
}

fn closed_receiver() {
    let (tx, rx) = mpsc::channel();

    drop(rx);
    let handle = thread::spawn(move || {
        if tx.send("Hello world!").is_err() {
            println!("There was an error sending message");
        }
    });
    handle.join().unwrap();
}

fn closed_transmitter() {
    let (tx, rx) = mpsc::channel::<String>();

    drop(tx);
    match rx.recv() {
        Ok(mes) => println!("Message: {}", mes),
        Err(err) => println!("There was an error receiving mesage. Error: {}", err)
    };
}

fn multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if tx.send("Hello world!").is_err() {
            println!("There was an error sending message");
        }

        if tx.send("Are you still there?").is_err() {
            println!("There was an error sending message");
        }

        if tx.send("Ok, bye!").is_err() {
            println!("There was an error sending message");
        }
    });

    for res in rx {
        println!("Message: {}", res);
    }

    println!("Finished reading messages. Most likely the transmitter closed");
}

fn multiple_transmitters() {
    let (tx, rx) = mpsc::channel();

    let tx_clone = tx.clone();
    thread::spawn(move || {
        for i in 1..5 {
            if tx_clone.send(format!("Thread 1: {}", i)).is_err() {
                println!("There was an error sending message");
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        for i in 1..5 {
            if tx.send(format!("Thread 2: {}", i)).is_err() {
                println!("There was an error sending message");
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    for res in rx {
        println!("Message: {}", res);
    }

    println!("Finished reading messages. Most likely the transmitter closed");
}

fn main() {
    single_message();
    closed_receiver();
    closed_transmitter();
    multiple_messages();
    multiple_transmitters();
}
