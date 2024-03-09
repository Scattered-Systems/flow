/*
    Appellation: orchestrator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::FnStateSpace;

pub struct Orchestrator<T> {
    store: FnStateSpace<T>,
}
