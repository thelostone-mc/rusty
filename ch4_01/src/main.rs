fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{}", full); 

    // This will error cause the ownership of first has been moved to full and first is no longer valid
    // println!("{}", first); 

    // To fix this, we can use clone to create a new copy of the string
    // Here you cannot use first_clone after moving ownership to full
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{}", full);
    println!("{}", first);
    // println!("{}", first_clone);

    let random_string = String::from("Hello, world!");
    print_length(&random_string);
    println!("{}", random_string);

    // Challenge 1: Write a function clone_name that accepts a String and returns a clone of it. 
    // Then, demonstrate calling this function with a String and print the original and the cloned string to verify that both are separate instances.
    println!("{}", clone_name(random_string.clone()));
    println!("{}", random_string);

    // Challenge 2: Write a function take_ownership that accepts a String and returns it. In the main function, create a String and pass it to take_ownership, then try to use the original String after ownership has been transferred. Observe the behavior.
    println!("{}", take_ownership(random_string));
    // println!("{}", random_string); // This will error because the ownership of random_string has been moved to take_ownership

}

fn take_ownership(name: String) -> String {
    name
}

fn print_length(s: &String) {
    println!("{} has length {}", s, s.len());
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn clone_name(name: String) -> String {
    name.clone()
}