enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // println!("Message line is {}", )
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Florida,
    California,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(192, 168, 0, 1);
    let loopback = IpAddr::V6(String::from("127.0.0.1"));

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let some_number = Some(5);
    let some_char = Some('e');

//    let no_num : Option<i32> = None;

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let quarter = Coin::Quarter(UsState::California);
    println!("Value: {}", value_in_cents(penny));
    println!("Nickel Value {}", value_in_cents(nickel));
    println!("Value: {}", value_in_cents(quarter));

    let five = Some(5);
    let sex = plus_one(five);
    let none = plus_one(None);
}

fn route(ip_kind: IpAddrKind) {

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            print!("Lucky penny! ");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 15,
        Coin::Quarter(state) => {
            print!("Quarter from the great state of: {:?}! ", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}