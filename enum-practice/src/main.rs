#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    //let home = IpAddr::V4(127, 0, 0, 1);
    //let loopback = IpAddr::V6(String::from("::1"));
    //println!("home     = {:?}", home);
    //println!("loopback = {:?}", loopback);
    //let some_number                = Some(5);
    //let some_string                = Some("a string");
    //let absent_number: Option<i32> = None;
    //println!("some number is {:?}", some_number);
    //println!("some string is {:?}", some_string);
    //println!("some absent_number is {:?}", absent_number);

    //==========================================================================
    //let five = Some(5);
    //let six = plus_one(five);
    //let none = plus_one(None);
    //println!("five = {:?}", five);
    //println!("six  = {:?}", six);
    //println!("none = {:?}", none);

    //==========================================================================
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }

}
