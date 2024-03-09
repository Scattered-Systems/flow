/*
    Appellation: engine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Runtime;
use super::context::EngineContext as Context;
use super::drivers::Driver;

use std::sync::Arc;

pub struct Engine {
    context: Context,
    driver: Driver,
    rt: Arc<Runtime>,
}

