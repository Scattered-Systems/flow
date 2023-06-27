/*
   Appellation: states <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # 
/// 
/// 
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Idle,
    Processing { transaction: String },
}
