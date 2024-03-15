/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use tokio::sync::mpsc;

pub struct Client {
    tasks: mpsc::Sender<String>,
}
