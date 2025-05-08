#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option <i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None, 
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter_alabama = Coin::Quarter(USState::Alabama);
    let quarter_alaska = Coin::Quarter(USState::Alaska);
    
    println!("Penny: {} cents", value_in_cents(penny));
    println!("Nickel: {} cents", value_in_cents(nickel));
    println!("Dime: {} cents", value_in_cents(dime));
    println!("Alabama Quarter: {} cents", value_in_cents(quarter_alabama));
    println!("Alaska Quarter: {} cents", value_in_cents(quarter_alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five plus one: {:?}", six.unwrap());
    println!("None plus one: {:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {println!("Move player {} spaces", num_spaces);}

    let dice_roll_2 = 9;
    match dice_roll_2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn reroll() {
        println!("Rerolling...");
    }

    let dice_roll_3 = 9;
    match dice_roll_3 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}
