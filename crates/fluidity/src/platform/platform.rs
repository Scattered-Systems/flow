/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::context::PlatformContext;
use super::rt::engine::Engine;

pub struct Platform {
    pub(crate) ctx: PlatformContext,
    pub(crate) engine: Engine,
}