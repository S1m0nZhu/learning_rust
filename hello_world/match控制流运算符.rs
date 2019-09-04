enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin:Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
//绑定值的模式
enum UsState {
    Alabam,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarterfrom {:?}", state);
            25
        },
    }
}

//匹配option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,//Rust中匹配时穷尽的，必须穷举到最后可能性使代码生效
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

//_通配符
fn match_control_flow() {
    let some_u8_value = 7;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
fn main() {
    match_control_flow();
}