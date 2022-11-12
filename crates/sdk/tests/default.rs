/*
    Appellation: default <test>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[cfg(test)]
#[test]
fn lib_compiles() {
    let f = |x: usize, y: usize, z: usize| (x * y) + z;
    let actual = f(10, 2, 10);
    let expected = 22;
    assert_eq!(actual, expected)
}
