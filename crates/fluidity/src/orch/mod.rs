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
                let new_state = output.clone();
                if new_state.stage.len() > i {
                    if new_state.stage[i] {
                        return new_state;
                    }
                    let mut next_state = func(new_state).unwrap();
                    next_state.stage[i] = next_state.proceed;
                    return next_state;
                }
                let mut next_state = func(new_state).unwrap();
                next_state.stage.push(next_state.proceed);
                next_state
            })
    }
}

#[cfg(test)]
mod tests {}
