#[derive(Debug)]
enum UsState{
    Alaska,
    Alabama,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main(){
    /* let cooin = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cent(cooin));

    println!("add {}", add(50, Some(2))); */


    let dice_roll= 5;
    match dice_roll{
        3 => println!("You got a fancy head"),
        6 => println!("You got a fancy tail"),
        _ => println!("You got {}", other),
    };
}

fn add(num:i32, num2:Option<i32>) -> i32{
    match num2{
        Some(i) => num +i,
        None => num
    }
}

fn value_in_cent(coin: Coin) -> u8{
      match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => {
            println!("State Quarter form Alaska");
            27
        },
        Coin::Quarter(UsState)=> {
            println!("State Quarter form {:?}", UsState);
            25
        }
        
      }
}