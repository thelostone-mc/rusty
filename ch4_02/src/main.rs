fn main() {
    ex1();
    ex2();
}

fn ex1() {
    let text = String::from("hello");

    let r1 = &text;
    let r2 = &text;

    println!("{}, {}", r1, r2);
}

fn ex2() {
    let mut greeting = String::from("Hello");
    append_exclamation(&mut greeting);
    println!("{}", greeting); // should print: Hello!
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}