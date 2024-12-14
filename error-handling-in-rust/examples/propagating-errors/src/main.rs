use rand::Rng;

fn change_result_type() -> Result<String, i8> {
    let r = do_something()?;
    if r {
        return Ok("All good".to_string());
    } else {
        return Ok("Not so good".to_string());
    }
}

fn do_something() -> Result<bool, i8> {
    let num: i8 = rand::thread_rng().gen_range(0..3);
    if num == 0 {
        return Ok(true);
    } else {
        return Err(num);
    }
}

fn main() {
    match change_result_type() {
        Ok(r) => {
            println!("{}", r);
        },
        Err(e) => {
            println!("The error is: {}", e);
        }
    }
}
