/*
   Appellation: status <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Status
//!
pub use self::{kinds::StatusCode, message::Status};

pub(crate) mod kinds;
pub(crate) mod message;

pub enum TemporalOrder {
    After,
    Before,
    During,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_message() {
        let mut status = Status::new("test", StatusCode::pending(0));
        let start = status.timestamp();
        assert_eq!(status.message(), "test");
        std::thread::sleep(std::time::Duration::from_secs(1));
        status.set_message("test2");
        assert_eq!(status.message(), "test2");
        assert_ne!(status.timestamp(), start);
    }
}
