/* fn main() {
    let config_max: Option<u8>= None;
    if let None = config_max{
        println!("The maximum is configured to be None");
    } 
} */

enum B{
    A1,
    A2,
}
enum A{
    State(B),
    State1,
}

fn main(){
    let config_max= A::State1;
    if let A::State(B) = config_max{
        println!("The maximum is configured to be None");
    }else{
        println!("The maximum is not configured to be None");
    }

    let config_max: Option<u8>= Some(5);
    match config_max {
          Some(7) => println!("The maximum is configured to be "),
          _ => println!("The maximum is not configured to be None"),
    }
}