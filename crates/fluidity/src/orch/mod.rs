/*
    Appellation: orch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{orchestrator::*, state::*};

pub(crate) mod orchestrator;
pub(crate) mod state;

pub mod chain;


pub trait Orchestrate {
    type State;

    fn execute(self, state: Self::State) -> Self::State;
}

impl<T> Orchestrate for Vec<fn(State<T>) -> State<T>> where T: Clone {
    type State = State<T>;

     fn execute(self, state: Self::State) -> Self::State {
        self.iter().enumerate().fold(state, |output, (i, func)| {
            let new_state = output.clone();
            if new_state.stage.len() > i {
                if new_state.stage[i] {
                    return new_state;
                }
                let mut next_state = func(new_state);
                next_state.stage[i] = next_state.proceed;
                return next_state;
            }
            let mut next_state = func(new_state);
            next_state.stage.push(next_state.proceed);
            next_state
        })
    }
}

#[cfg(test)]
mod tests {}