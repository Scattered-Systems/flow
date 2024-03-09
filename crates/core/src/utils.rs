/*
   Appellation: utils <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/

pub fn systime() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
