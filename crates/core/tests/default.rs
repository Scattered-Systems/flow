/*
    Appellation: default <test>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[cfg(test)]
use fluidity_core::State;

#[test]
fn test_state() {
    let actual = State::default();
    let expected = actual.clone();
    assert_eq!(actual, expected)
}