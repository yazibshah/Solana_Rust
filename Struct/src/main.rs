/* 
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    
    let user1= User{
        active: true,
        username: String::from("yazib"),
        email: String::from("yazib@gmail.com"),
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username);
    let user2= User{
        active: false,
        username: String::from("Ali"),
        email: String::from("ali@gmail.com"),
        sign_in_count: 0,
    };
}
 */


// Mut 

/* struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} */

/* fn main() {
    
    let mut user1= User{
        active: true,
        username: String::from("yazib"),
        email: String::from("yazib@gmail.com"),
        sign_in_count: 1,
    };

    user1.username=String::from("Azlan");
    user1.username.push_str(" Shah");
    println!("Username: {}", user1.username);
    
} */
/* 
fn main() {
    
    let mut user1= User{
        active: true,
        username: String::from("yazib"),
        email: String::from("yazib@gmail.com"),
        sign_in_count: 1,
    };

    let s1=user1.username;
    // println!("Username: {}", user1.username);

    user1.username= String::from("jone");
    println!("Username: {}", user1.username);
} */


/* struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    
    let mut user1= build_user(String::from("yazib"), String::from("yazib@gmail.com"));
    println!("Username: {} | sign_count: {}", user1.username, user1.sign_in_count);

    let mut user2= build_user(String::from("Ali"), String::from("Ali@gmail.com"));
    println!("Username: {} | sign_count: {}", user2.username, user2.sign_in_count);
}

fn build_user(username:String, email:String) -> User{
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
} */



/* struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    
    let mut user1= build_user(String::from("yazib"), String::from("yazib@gmail.com"));
    println!("Username: {} | sign_count: {}", user1.username, user1.sign_in_count);

    let mut user2= User{
        email: String::from("jhon@gmail.com"),
        ..user1
    };
    println!("Username: {} | email: {}", user2.username, user2.email);
    user1.username=String::from("NewName");
    println!("Username: {}", user1.username);
    
    
}

fn build_user(username:String, email:String) -> User{
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
} */


/* fn main(){
    let red= (255, 0, 0);
    set_color(red);

}

fn set_color(color:(u8, u8, u8)) {
    println!("Red: {}, Green: {}, Blue: {}", color.0, color.1, color.2);
} */


struct Color(u8, u8, u8);
struct Point(u8,u8,u8);


fn main(){
    let black= Color(24, 7, 9);
    let point = Point(10,20, 30);

    set_color(black);
    set_point(black);


}
fn set_color(color:Color){
    println!("Color Values: {}, {}, {}", color.0, color.1, color.2);
}
fn set_point(point:Point){
    println!("Point Values: {}, {}, {}", point.0, point.1, point.2);
}