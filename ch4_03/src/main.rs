use std::rc::Rc;

fn main() {
    // println!("{}", incorrect_return_a_string());
    println!("{}", return_a_string_method_as_static_str());
    println!("{}", return_a_string_method_with_rc());
}

// fn incorrect_return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

fn return_a_string_method_as_static_str() -> &'static str {
    "Hello world"
}

fn return_a_string_method_with_rc() -> Rc<String> {
    let s = Rc::new(String::from("Hello world")); // count = 1
    Rc::clone(&s) // count = 2
} // s is dropped here, count = 1