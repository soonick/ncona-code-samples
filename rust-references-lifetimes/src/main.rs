struct HoldReference<'a> {
    something: &'a str,
}

fn return_best<'a>(input1: &'a str, input2: &'a str) -> &'a str {
    if input1 > input2 {
        input1
    } else {
        input2
    }
}

fn function_ref() {
    let str1 = String::from("One");
    let str2 = String::from("Two");
    let str3 = return_best(&str1, &str2);
    println!("{}", str3);
}

fn struct_ref() {
    let str1 = String::from("One");
    let st = HoldReference{
        something: &str1
    };
    println!("{}", st.something);

}

fn main() {
    function_ref();
    struct_ref();
}
