#[derive(Debug)]
enum UsState {
    Alaska,
    NY,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState{
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1819,
            UsState::NY => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    }; 

    if state.existed_in(1900){
        Some(format!("{state:?} is pretty old"))
    } else {
        Some(format!("{state:?} is pretty new"))
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    println!("Hello, world!");
    let c = Coin::Quarter(UsState::Alaska);
    
    println!("{}", value_in_cents(&c));

    println!("{:?}", plus_one(None));
    println!("{:?}", plus_one(Some(5)));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
    

    //if let
    let mut count = 0;
    if let Coin::Quarter(state) = &c {
        println!("{state:?}");
    }
    else {
        count += 1;
    }

    println!("{count}");

    if let Some(val) = describe_state_quarter(c) {
        println!("{val}");
    }

}
