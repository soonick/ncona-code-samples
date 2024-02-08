trait Calculator {
    fn add(&self, left: i32, right: i32) -> i32;
}

trait SoundMaker {
    fn make_sound(&self) {
        println!("")
    }
}

fn add_using_calculator(calculator: &impl Calculator) {
    println!("The result of adding {} and {} is: {}", 10, 5, calculator.add(10, 5));
}

fn make_sound_and_add(the_thing: &(impl Calculator + SoundMaker)) {
    the_thing.make_sound();
    println!("The result of adding {} and {} is: {}", 10, 5, the_thing.add(10, 5));
}

fn make_sound_and_add_2<T: Calculator + SoundMaker>(the_thing: &T) {
    the_thing.make_sound();
    println!("The result of adding {} and {} is: {}", 10, 5, the_thing.add(10, 5));
}

fn make_sound_and_add_3<T>(the_thing: &T)
where
    T: Calculator + SoundMaker
{
    the_thing.make_sound();
    println!("The result of adding {} and {} is: {}", 10, 5, the_thing.add(10, 5));
}

struct GoodCalculator {}

impl Calculator for GoodCalculator {
    fn add(&self, left: i32, right: i32) -> i32 {
        left + right
    }
}

struct Human {}

impl Calculator for Human {
    fn add(&self, left: i32, right: i32) -> i32 {
        left + right
    }
}

impl SoundMaker for Human {
    fn make_sound(&self) {
        println!("Hello!");
    }
}

fn main() {
    let calc = GoodCalculator{};
    add_using_calculator(&calc);
    let human = Human{};
    make_sound_and_add(&human);
    make_sound_and_add_2(&human);
    make_sound_and_add_3(&human);
}
