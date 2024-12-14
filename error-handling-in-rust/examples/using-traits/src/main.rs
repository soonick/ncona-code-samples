use std::env;

fn do_something() -> Result<bool, Box<dyn std::error::Error>> {
    env::current_dir()?;
    Ok(true)
}

fn main() {
    match do_something() {
        Ok(r) => {
            println!("{}", r);
        },
        Err(e) => {
            println!("The error is: {}", e.to_string());
        }
    }
}
