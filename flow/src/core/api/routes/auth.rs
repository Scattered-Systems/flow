/*
   Appellation: auth <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

pub enum Login {
    Std {
        username: String,
        password: String
    }
}

pub fn auth(data: Login) -> bool {
    
}