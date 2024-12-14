use std::env;

fn main() {
    match env::current_dir() {
        Ok(t) => {
            println!("The current directory is: {}", t.display());
        },
        Err(e) => {
            println!("There was an error: {}", e);
        }
    }

    let res = env::current_dir();
    match res {
        Ok(t) => {
            println!("The current directory is: {}", t.display());
        },
        Err(e) => {
            println!("There was an error: {}", e);
        }
    }

}
