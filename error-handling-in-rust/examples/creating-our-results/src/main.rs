use rand::Rng;

fn do_something() -> Result<bool, i8> {
    let num: i8 = rand::thread_rng().gen_range(0..3);
    if num == 0 {
        return Ok(true);
    } else {
        return Err(num);
    }
}

fn main() {
    match do_something() {
        Ok(r) => {
            println!("The result is: {}", r);
        },
        Err(e) => {
            println!("The error is: {}", e);
        }
    }
}
