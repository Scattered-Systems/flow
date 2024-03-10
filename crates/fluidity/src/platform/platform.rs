/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::context::PlatformContext;
use std::collections::HashMap;

pub enum PlatformMode {
    AccessPoint,
    Node,
}

pub struct Platform {
    ctx: PlatformContext,
    mode: PlatformMode,
    state: String,
    tasks: HashMap<String, String>,
}
