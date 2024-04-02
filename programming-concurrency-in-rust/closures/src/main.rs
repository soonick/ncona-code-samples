fn borrow_number() {
    let num = 1;
    let closure = || println!("The number is {}", num);
    println!("Closure not executed yet");
    closure();
}

fn move_number() -> impl Fn() {
    let num = 2;
    move || println!("The number is {}", num)
}

fn integer_is_copied() -> impl Fn() {
    let num = 3;
    let closure = move || println!("The number is {}", num);
    println!("after moving integer: {}", num);
    closure
}

fn closure_with_args() {
    let num = 4;
    let closure =
        |another_num| println!("Captured number: {} Argument number: {}", num, another_num);
    closure(5);
}

fn different_syntax() {
    let c1 = || println!("c1");
    let c2 = |num| println!("c2: {}", num);
    let c3 = |num1, num2| {
        println!("c3: {}", num1);
        println!("c3: {}", num2);
    };
    let c4 = || 45; // We are returning this value
    let c5 = |num: i32| -> i32 {
        println!("c5: {}", num);
        99
    };
    c1();
    c2(1);
    c3(2, 3);
    c4();
    c5(4);
}

fn borrowed_mutably() {
    let mut num = 5;
    println!("The number is {}", num);
    let mut closure = || {
        num += 1;
        println!("The number inside closure is {}", num);
    };
    closure();
    println!("The number after closure execution is {}", num);
}

fn main() {
    borrow_number();
    move_number()();
    integer_is_copied()();
    closure_with_args();
    different_syntax();
    borrowed_mutably();
}
