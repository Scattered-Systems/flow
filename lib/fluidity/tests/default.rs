#[cfg(test)]
mod tests {

    #[test]
    fn lib_compiles() {
        let f = | i: usize, j: usize | i + j;
        let a = f(2, 2);
        assert_eq!(a, 4);
    }
}
