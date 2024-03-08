/*
   Appellation: error <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::ErrorKind;

pub struct Error {
    kind: ErrorKind,
    message: String,
}