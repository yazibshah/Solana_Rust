/* fn main() {
    let mut s:String= String::from("Hello world");
    println!("{}", s);

    s.push_str("!!!");
    println!("{}", s);
} */


/* fn main(){
    let x=5;
    let y=x;

    println!("{}",x);
    println!("{}",y);
} */

/* fn main(){
    let x=String::from("I'm x");
    let y=x;

    println!("{}",x);
    println!("{}",y);
} */

fn main(){
    let x=String::from("I'm x");
    let mut y=x.clone();
    y.push_str("!!");
    println!("{}",x);
    println!("{}",y);
}