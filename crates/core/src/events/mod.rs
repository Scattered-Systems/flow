/*
   Appellation: errors <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::event::*;

pub(crate) mod event;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_event() {
        assert!(true);
    }
}
