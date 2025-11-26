/* fn main(){
    let mut s=String::from("Hello");
    s.push_str("world");

    let length=calculate_length(&mut s);
    println!("{length}");
    println!("{s}");
}

fn calculate_length(s:&mut String) -> usize{
    s.push_str(" !!!");
    s.len()
} */


/* fn main(){
    let mut s1= String::from("Hello");

    let s2= &mut s1;
    let s3= &mut s1;

    s2.push_str("world");
    println!("{}",s1);
} */


/* fn main(){
    let mut s1= String::from("Hello");

    let s2= &mut s1;
    let s3= s1.len();

    s2.push_str("world");
    println!("{}",s1);

} */

/* fn main(){
    let mut s1=String::from("Hello");
    {
        let s2=&mut s1;
        s2.push_str(" world");
        println!("{}",s2);
    }
    let s3=&mut s1;
    s3.push_str("!!!");
    println!("{}",s1);
} */


// fn main(){
//     let mut s1= String::from("Hello");

//     let s2= &s1;
//     println!("{}",s2);
//     let s3= &mut s1;
//     s3.push_str(" world");
//     // println!("{} , {}",s2,s3); // error
//     println!("{}",s3);
// }

// Dangling Reference

fn main(){
    // let refrence_nothing= dengle(); // error of dengle.

    let no_refrence_nothing2= no_dengle();
}

/* fn dengle() -> &String{
    let s=String::from("hello");
    &s
} */  // error

fn no_dengle() -> String{
    let s=String::from("hello");
    s
}