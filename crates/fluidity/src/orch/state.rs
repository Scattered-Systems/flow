/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;

pub(crate) type TaskFlow = ControlFlow<(), ()>;

pub trait TaskState<T> {
    fn output(&self) -> Option<T>;

    fn flow(&self) -> TaskFlow;
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct State<T = ()> {
    pub output: Option<T>,
    pub proceed: bool,
    pub stage: Vec<bool>,
}

impl<T> State<T> {
    pub fn new(output: Option<T>, proceed: bool, stage: impl IntoIterator<Item = bool>) -> Self {
        State {
            output,
            proceed,
            stage: Vec::from_iter(stage),
        }
    }

    pub fn output(&self) -> Option<&T> {
        self.output.as_ref()
    }

    pub fn flow(&self) -> TaskFlow {
        if self.proceed {
            TaskFlow::Continue(())
        } else {
            TaskFlow::Break(())
        }
    }

    pub fn proceed(&self) -> bool {
        self.proceed
    }

    pub fn stage(&self) -> &[bool] {
        &self.stage
    }
}

pub struct Stage {
    pub(crate) stage: bool,
}

impl Stage {
    pub fn new(stage: bool) -> Self {
        Self { stage }
    }

    pub fn stage(&self) -> bool {
        self.stage
    }

    pub fn set_stage(&mut self, stage: bool) {
        self.stage = stage
    }
}
