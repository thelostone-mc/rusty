use ex1::NumberData;

fn main() {
    let numbers = NumberData::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("The median is {:?}", numbers.median());
    println!("The mode is {:?}", numbers.mode());
}
