use std::fmt;

#[derive(Debug, Clone)]
struct OurError;

impl fmt::Display for OurError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Just a custom error")
    }
}

impl std::error::Error for OurError {}

fn do_something() -> Result<bool, Box<dyn std::error::Error>> {
    Err(Box::new(OurError))
}

fn main() {
    match do_something() {
        Ok(_) => {
            println!("This won't happen");
        },
        Err(e) => {
            println!("The error is: {}", e.to_string());
        }
    }
}
