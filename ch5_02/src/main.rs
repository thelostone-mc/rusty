#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect1);
    println!("The area of the rectangle with dimenstions {rect1:?} is {} square pixels.", calculate_area(&rect1));
}
