fn main() {
    let word = String::from("apple");
    let pig_latin = to_pig_latin(word);
    println!("{}", pig_latin);

    let word2 = String::from("first");
    let pig_latin2 = to_pig_latin(word2);
    println!("{}", pig_latin2);

    let word3 = String::from("学");
    let pig_latin3 = to_pig_latin(word3);
    println!("{}", pig_latin3);
}

fn to_pig_latin(word: String) -> String {

    let mut chars = word.chars();

    match chars.next() {
        Some(c) if is_vowel(c) => format!("{}-hay", word),
        Some(c) => {
            let rest: String = chars.collect();
            format!("{}-{}ay", rest, c)
        }
        None => String::new(),
    }
}

fn is_vowel(char: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    vowels.contains(&char.to_ascii_lowercase())
}
