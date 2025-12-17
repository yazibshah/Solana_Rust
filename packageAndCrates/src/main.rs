use packageAndCrates::{auth_utils::models::Credentials, authenticate};
// use packageAndCrates::;


fn main(){
    let cred: Credentials = Credentials{
        username: String::from("yazib"),
        password: String::from("password123")
    };

    authenticate(cred); 
    // packageAndCrates::authenticate(cred);
}