#[cfg(test)]

#[test]
fn compiles() {
    let f = |i: usize, j: usize| i + j;
    let a = f(2, 2);
    assert_eq!(a, 4);
}
