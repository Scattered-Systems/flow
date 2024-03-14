/*
    Appellation: orch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{orchestrator::*, state::*};

pub(crate) mod orchestrator;
pub(crate) mod state;

pub mod chain;

use crate::core::prelude::Result;

pub(crate) type StateFn<T> = fn(State<T>) -> Result<State<T>>;
pub(crate) type FnStateSpace<T> = Vec<StateFn<T>>;

pub trait Orchestrate {
    type State;

    fn orchestrate(self, state: Self::State) -> Self::State;
}

impl<I, T> Orchestrate for I
where
    I: IntoIterator<Item = StateFn<T>>,
    T: Clone,
{
    type State = State<T>;

    fn orchestrate(self, state: Self::State) -> Self::State {
        self.into_iter()
            .enumerate()
            .fold(state, |output, (i, func)| {
                if output.stage().len() > i {
                    if output.stage[i] {
                        return output;
                    }
                    let mut next = func(output).unwrap();
                    next.stage[i] = next.proceed();
                    return next;
                }
                let mut next = func(output).unwrap();
                next.stage.push(next.proceed());
                next
            })
    }
}

#[cfg(test)]
mod tests {}
