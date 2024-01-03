enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddr{
    kind: IpAddrKind,
    address: String
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
    // You can also define each of the variants as a different struct but, these structs will be all of different type, hence the enum type can have all these variants under the same type.
}

impl Message{
    fn some_fn(){
        println!("Some fn");
        // You can also implement functions for enum.
    }
}

pub fn main(){
    let localhost = IpAddrKind::V4(128, 0, 0, 1);

    let some_num = Some(4);
    let some_str = Some("some string");
    let no_num: Option<i32> = None;
    // Rust doesn't have null value, so to handle that Option enum is always integrated in the code.

    let x = 5;
    let y = Some(5);

    // You cannot add Option<i32> with i32, so to handle this error, you unwrap the variable
    let sum = x + y.unwrap_or(0);    // Unwrap can be of many types, the "unwrap_or" means either the value of y if exists or the default value.

    // Enum inside Enum
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));


    // match some_num{
    //     Some(4) => println!("Four"),
    //     _ => ()
    // }
    // if we want to check only for a specific value, unlike match statement where we need to take care of all the cases, use if let.
    if let Some(4) = some_num{
        println!("Four");
    }

}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
    //...
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State: {:?}", state);
            25
        }
    }
}