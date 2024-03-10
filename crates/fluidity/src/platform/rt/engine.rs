/*
    Appellation: engine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::drivers::Driver;
use super::Runtime;

use std::sync::Arc;

pub struct Engine {
    driver: Driver,
    rt: Arc<Runtime>,
}
