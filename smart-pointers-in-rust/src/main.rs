use std::rc::Rc;

fn expensive_move() {
    let text = String::from("Some text");
    println!("The text is at: {:p}", &text);
    let new_text = text; // This invalidates text
    println!("The text is at: {:p}", &new_text);
}

fn box_move() {
    let b1 = Box::new(String::from("Another text"));
    println!("b1 is at {:p}, b1 points to: {:p}. The text is: {}", &b1, b1, b1);
    let b2 = b1;
    println!("b2 is at {:p}, b2 points to: {:p}. The text is: {}", &b2, b2, b2);
}

fn rc_pointer() {
    let pointer = Rc::new(String::from("More text"));
    // Reference count here is 1
    println!("pointer is at {:p}, pointer points to: {:p}, reference count is: {}", &pointer, pointer, Rc::strong_count(&pointer));
    {
        let pointer_clone = Rc::clone(&pointer);
        // pointer_clone is at a different address than pointer, but the underlying
        // data is at the same address. Reference count here is 2
        println!("pointer is at {:p}, pointer points to: {:p}, reference count is: {}", &pointer_clone, pointer_clone, Rc::strong_count(&pointer_clone));

        // Reference count here is 2
        println!("pointer is at {:p}, pointer points to: {:p}, reference count is: {}", &pointer, pointer, Rc::strong_count(&pointer));
    }

    // Since pointer_clone has gone out of scope, reference count was decreased to 1
    println!("pointer is at {:p}, pointer points to: {:p}, reference count is: {}", &pointer, pointer, Rc::strong_count(&pointer));
}

fn main() {
    expensive_move();
    box_move();
    rc_pointer();
}
