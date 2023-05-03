#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_of_coin(c: Coin) -> isize {
    match c {
        Coin::Penny => {
            println!("this is Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let c = dbg!(Coin::Quarter);
    println!("coin is {}", value_of_coin(c));
}
