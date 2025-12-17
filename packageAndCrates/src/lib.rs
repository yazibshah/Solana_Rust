pub mod util;

mod database;

 
pub mod auth_utils;
use auth_utils::{login, models::Credentials};
use database::Status;

pub fn authenticate(cred: Credentials){
    if let Status::Connected = database::connect_to_database(){
        println!("User {} authenticated successfully.", cred.username);
        login(cred);
    }else{
        println!("Authentication failed due to connection issues.");
    }
}