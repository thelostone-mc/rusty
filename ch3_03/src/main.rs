fn another_name(x: i32) -> i32 {
    x + 1
}
fn main() {
    let y = another_name(5);

    let z = {
        let x = 1;
        x
    };
    println!("{}", y);
    println!("{}", z);

    let t1 = if y > 3 { 2 } else { 3 };
    println!("{}", t1);

}

