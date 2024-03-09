/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};


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


}



