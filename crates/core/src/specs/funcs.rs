/*
    Appellation: funcs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/


pub trait FunctionSpace<Args> {
    type Output;

    fn call(&self, args: Args) -> Self::Output;
}