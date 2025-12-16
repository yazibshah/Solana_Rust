pub struct Credentials{
    pub username: String,
    pub password: String,
}

enum Status{
    Connected,
    Interrupted,
}

pub fn connect_to_database() -> Status {
    Status::Connected
}

pub fn get_user() {

}

pub fn login(cred:Credentials){
    get_user();
}
pub fn authenticate(cred: Credentials){
    if let Status::Connected = connect_to_database(){
        println!("User {} authenticated successfully.", cred.username);
    }else{
        println!("Authentication failed due to connection issues.");
    }
}