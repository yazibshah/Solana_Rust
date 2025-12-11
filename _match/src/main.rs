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
    let cooin = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cent(cooin));
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
        Coin::Quarter(state) => {
            println!("State Quarter form {:?}", state);
            25
        }
        
      }
}