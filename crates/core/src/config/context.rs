/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Contextual {}

pub trait Context: Send + Sync {
    type Config;

    fn settings(&self) -> Self::Config;
}
