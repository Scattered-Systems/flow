/*
    Appellation: engine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::context::Context;
use super::drivers::Driver;

pub struct Engine {
    context: Context,
    driver: Driver
}

