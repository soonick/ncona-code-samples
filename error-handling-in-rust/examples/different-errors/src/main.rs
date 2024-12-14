use std::fmt;

// CustomErrorEnum
#[derive(Debug, Clone)]
enum CustomErrorEnum {
    OurError(OurError),
    ErrorTwo(ErrorTwo),
}

// OurError
#[derive(Debug, Clone)]
struct OurError;

impl fmt::Display for OurError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Just a custom error")
    }
}

impl std::error::Error for OurError {}

// ErrorTwo
#[derive(Debug, Clone)]
struct ErrorTwo;

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Another custom error")
    }
}

impl std::error::Error for ErrorTwo {}


fn do_something() -> Result<bool, CustomErrorEnum> {
    Err(CustomErrorEnum::ErrorTwo(ErrorTwo))
}

fn main() {
    match do_something() {
        Ok(_) => {
            println!("This won't happen");
        },
        Err(CustomErrorEnum::OurError(e)) => {
            println!("Got OurError: {}", e);
        },
        Err(CustomErrorEnum::ErrorTwo(e)) => {
            println!("Got ErrorTwo: {}", e);
        },
    }
}
